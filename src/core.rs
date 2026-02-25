use pyo3::Py;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::pyclass;
use pyo3::pymethods;
use pyo3::types::PyAnyMethods;
use tokio::runtime::Handle;

pyo3::create_exception!(gxhash_py, GxHashAsyncError, pyo3::exceptions::PyException);

#[cfg_attr(Py_3_10, pyclass(frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(frozen))]
pub struct GxHash32 {
    seed: i64,
    runtime: Handle,
}

#[cfg_attr(Py_3_10, pyclass(frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(frozen))]
pub struct GxHash64 {
    seed: i64,
    runtime: Handle,
}

#[cfg_attr(Py_3_10, pyclass(frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(frozen))]
pub struct GxHash128 {
    seed: i64,
    runtime: Handle,
}

#[cfg_attr(Py_3_10, pyclass(frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(frozen))]
struct TokioRuntime {
    runtime: tokio::runtime::Runtime,
}

#[pymethods]
impl TokioRuntime {
    #[new]
    fn new() -> PyResult<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread().build()?;
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
                    .import("gxhash.core")?
                    .getattr("runtime")?
                    .extract::<pyo3::PyRef<TokioRuntime>>()?
                    .runtime
                    .handle()
                    .clone();

                Ok(Self { seed, runtime })
            }

            #[pyo3(signature = (data, /))]
            fn hash(&self, data: &[u8]) -> $return_type {
                $hasher(data, self.seed)
            }

            #[pyo3(signature = (data, /))]
            async fn hash_async(&self, data: Py<pyo3::types::PyBytes>) -> PyResult<$return_type> {
                let seed = self.seed;
                let bytes_slice = Python::attach(|py| data.as_bytes(py));

                match bytes_slice.len() < 4 << 20 {
                    true => Ok($hasher(bytes_slice, seed)),
                    false => self
                        .runtime
                        .spawn_blocking(move || $hasher(Python::attach(|py| data.as_bytes(py)), seed))
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

/// Core Python bindings for GxHash
///
/// This module contains the core Python bindings for GxHash, a blazingly fast and robust non-cryptographic hashing algorithm.
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
#[pyo3::pymodule(submodule, name = "core", gil_used = false)]
pub mod core_module {
    use pyo3::prelude::PyModuleMethods;

    #[pymodule_export]
    use super::GxHash32;
    #[pymodule_export]
    use super::GxHash64;
    #[pymodule_export]
    use super::GxHash128;
    #[pymodule_export]
    use super::GxHashAsyncError;

    #[pymodule_init]
    fn init(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
        let py = m.py();
        m.add("runtime", pyo3::Py::new(py, super::TokioRuntime::new()?)?)
    }
}
