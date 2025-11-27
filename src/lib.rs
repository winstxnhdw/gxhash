use pyo3::prelude::Bound;
use pyo3::prelude::PyResult;
use pyo3::prelude::Python;
use pyo3::prelude::pyclass;
use pyo3::prelude::pymethods;

pyo3::create_exception!(gxhash_py, GxHashAsyncError, pyo3::exceptions::PyException);

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
struct GxHash32 {
    seed: i64,
}

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
struct GxHash64 {
    seed: i64,
}

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen))]
struct GxHash128 {
    seed: i64,
}

macro_rules! impl_gxhash_methods {
    ($Self:ident, $return_type:ty, $hasher:path) => {
        #[pymethods]
        impl $Self {
            #[new]
            fn new(seed: i64) -> Self {
                $Self { seed }
            }

            fn hash(&self, bytes: &[u8]) -> $return_type {
                $hasher(bytes, self.seed)
            }

            fn hash_async<'a>(
                &self,
                py: Python<'a>,
                bytes: pyo3::prelude::Py<pyo3::types::PyBytes>,
            ) -> PyResult<Bound<'a, pyo3::prelude::PyAny>> {
                let seed = self.seed;

                pyo3_async_runtimes::tokio::future_into_py(py, async move {
                    tokio::task::spawn_blocking(move || $hasher(Python::attach(|py| bytes.as_bytes(py)), seed))
                        .await
                        .map_err(|e| GxHashAsyncError::new_err(e.to_string()))
                })
            }
        }
    };
}

impl_gxhash_methods!(GxHash32, u32, gxhash::gxhash32);
impl_gxhash_methods!(GxHash64, u64, gxhash::gxhash64);
impl_gxhash_methods!(GxHash128, u128, gxhash::gxhash128);

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type, subclass))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen, subclass))]
struct Hasher {}

#[pymethods]
impl Hasher {
    #[new]
    fn new() -> Self {
        Self {}
    }
}

/// gxhash — Python bindings for gxhash
///
/// This module contains the Python bindings for GxHash, a blazingly fast and robust non-cryptographic hashing algorithm.
///
/// * GxHash32*:  a class for computing 32-bit hashes
/// * GxHash64*:  a class for computing 64-bit hashes
/// * GxHash128*: a class for computing 128-bit hashes
///
/// Each class provides methods for hashing byte sequences both synchronously and asynchronously.
///
/// * hash(bytes: bytes) -> int
/// * hash_async(bytes: bytes) -> Awaitable[int]
///
#[pyo3::prelude::pymodule(name = "gxhash")]
mod gxhash_py {
    #[pymodule_export]
    use super::GxHash32;
    #[pymodule_export]
    use super::GxHash64;
    #[pymodule_export]
    use super::GxHash128;
    #[pymodule_export]
    use super::GxHashAsyncError;
    #[pymodule_export]
    use super::Hasher;
}
