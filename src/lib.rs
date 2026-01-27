mod hashlib;

use pyo3::PyResult;
use pyo3::Python;
use pyo3::pyclass;
use pyo3::pymethods;
use pyo3::types::PyAnyMethods;
use tokio::runtime::Handle;

pyo3::create_exception!(gxhash_py, GxHashAsyncError, pyo3::exceptions::PyException);

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
pub struct GxHash32 {
    seed: i64,
    runtime: Handle,
}

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
pub struct GxHash64 {
    seed: i64,
    runtime: Handle,
}

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
pub struct GxHash128 {
    seed: i64,
    runtime: Handle,
}

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
struct TokioRuntime {
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl TokioRuntime {
    #[new]
    fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread().build()?;
        let worker_count = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(worker_count));

        runtime.block_on(futures_util::future::join_all((0..worker_count).map(|_| {
            let barrier = barrier.clone();
            runtime.spawn_blocking(move || barrier.wait())
        })));

        Ok(Self { runtime })
    }
}

macro_rules! impl_gxhash_methods {
    ($name:ident, $return_type:ty, $hasher:path) => {
        #[pymethods]
        impl $name {
            #[new]
            fn new(py: Python, seed: i64) -> PyResult<Self> {
                let runtime = py
                    .import("gxhash")?
                    .getattr("runtime")?
                    .extract::<pyo3::PyRef<TokioRuntime>>()?
                    .runtime
                    .handle()
                    .clone();

                Ok(Self { seed, runtime })
            }

            fn hash(&self, bytes: &[u8]) -> $return_type {
                $hasher(bytes, self.seed)
            }

            async fn hash_async(&self, bytes: pyo3::Py<pyo3::types::PyBytes>) -> PyResult<$return_type> {
                let seed = self.seed;
                let bytes_slice = Python::attach(|py| bytes.as_bytes(py));

                match bytes_slice.len() < 4 * 1024 * 1024 {
                    true => Ok($hasher(bytes_slice, seed)),
                    false => self
                        .runtime
                        .spawn_blocking(move || $hasher(Python::attach(|py| bytes.as_bytes(py)), seed))
                        .await
                        .map_err(|e| GxHashAsyncError::new_err(e.to_string())),
                }
            }
        }
    };
}

impl_gxhash_methods!(GxHash32, u32, gxhash_core::gxhash32);
impl_gxhash_methods!(GxHash64, u64, gxhash_core::gxhash64);
impl_gxhash_methods!(GxHash128, u128, gxhash_core::gxhash128);

/// gxhash — Python bindings for gxhash
///
/// This module contains the Python bindings for GxHash, a blazingly fast and robust non-cryptographic hashing algorithm.
///
/// * GxHash32  - a class for computing 32-bit hashes
/// * GxHash64  - a class for computing 64-bit hashes
/// * GxHash128 - a class for computing 128-bit hashes
///
/// Each class provides methods for hashing byte sequences both synchronously and asynchronously.
///
/// * GxHash32(seed: int)
/// * hash(bytes: bytes) -> Uint32
/// * hash_async(bytes: bytes) -> Awaitable[Uint32]
///
/// * GxHash64(seed: int)
/// * hash(bytes: bytes) -> Uint64
/// * hash_async(bytes: bytes) -> Awaitable[Uint64]
///
/// * GxHash128(seed: int)
/// * hash(bytes: bytes) -> Uint128
/// * hash_async(bytes: bytes) -> Awaitable[Uint128]
///
#[pyo3::pymodule(name = "gxhash", gil_used = false)]
pub mod gxhash_py {
    use pyo3::prelude::PyModuleMethods;
    use pyo3::types::PyAnyMethods;

    #[pymodule_export]
    use super::GxHash32;
    #[pymodule_export]
    use super::GxHash64;
    #[pymodule_export]
    use super::GxHash128;
    #[pymodule_export]
    use super::GxHashAsyncError;
    #[pymodule_export]
    use super::hashlib::hashlib_module;

    #[pymodule_init]
    fn init(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
        let py = m.py();
        m.add("runtime", pyo3::Py::new(py, super::TokioRuntime::new()?)?)?;
        py.import("sys")?
            .getattr("modules")?
            .set_item("gxhash.hashlib", m.getattr("hashlib")?)
    }
}
