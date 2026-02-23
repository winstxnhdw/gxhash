use crate::buffer::PyBufferExt;

use pyo3::Py;
use pyo3::PyResult;
use pyo3::buffer::PyBuffer;
use pyo3::ffi;
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
            fn new(py: pyo3::Python, seed: i64) -> PyResult<Self> {
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
            fn hash(&self, data: pyo3::Bound<pyo3::PyAny>) -> $return_type {
                let mut view = std::mem::MaybeUninit::<pyo3::ffi::Py_buffer>::uninit();

                unsafe {
                    ffi::PyObject_GetBuffer(data.as_ptr(), view.as_mut_ptr(), ffi::PyBUF_SIMPLE);
                    let mut view = view.assume_init();

                    let result = $hasher(
                        std::slice::from_raw_parts(view.buf as *const u8, view.len as usize),
                        self.seed,
                    );

                    ffi::PyBuffer_Release(&mut view);
                    result
                }
            }

            #[pyo3(signature = (data, /))]
            async fn hash_async(&self, data: PyBuffer<u8>) -> PyResult<$return_type> {
                let seed = self.seed;

                match data.len_bytes() < 4 << 20 {
                    true => Ok($hasher(data.as_bytes(), seed)),
                    false => self
                        .runtime
                        .spawn_blocking(move || $hasher(data.as_bytes(), seed))
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
