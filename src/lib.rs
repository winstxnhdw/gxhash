use pyo3::prelude::Py;
use pyo3::prelude::PyResult;
use pyo3::prelude::Python;
use pyo3::prelude::pyclass;
use pyo3::prelude::pymethods;
use pyo3::types::PyAnyMethods;
use tokio::runtime::Handle;

pyo3::create_exception!(gxhash_py, GxHashAsyncError, pyo3::exceptions::PyException);

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type, subclass))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(frozen, subclass))]
struct Hasher;

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
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .build()
            .expect("Unable to construct runtime!");

        Ok(Self { runtime })
    }
}

#[pymethods]
impl Hasher {
    #[new]
    #[pyo3(signature = (**_kwargs))]
    fn new(_kwargs: Option<Py<pyo3::types::PyDict>>) -> PyResult<Self> {
        let error = pyo3::exceptions::PyTypeError::new_err(r#"Cannot instantiate Protocol class "Hasher""#);
        Err(error)
    }

    #[classmethod]
    pub fn __class_getitem__(_cls: Py<pyo3::types::PyType>, _key: Py<pyo3::PyAny>) {}
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

            async fn hash_async(&self, bytes: pyo3::prelude::Py<pyo3::types::PyBytes>) -> PyResult<$return_type> {
                let seed = self.seed;

                self.runtime
                    .spawn_blocking(move || $hasher(Python::attach(|py| bytes.as_bytes(py)), seed))
                    .await
                    .map_err(|e| GxHashAsyncError::new_err(e.to_string()))
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
#[pyo3::prelude::pymodule(name = "gxhash", gil_used = false)]
pub mod gxhash_py {
    use pyo3::IntoPyObjectExt;
    use pyo3::prelude::PyModuleMethods;
    use pyo3::types::IntoPyDict;
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
    use super::Hasher;

    #[pymodule_init]
    fn init(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
        let py = m.py();
        let int = py.import("builtins")?.getattr("int")?;
        let typevar = py.import("typing")?.getattr("TypeVar")?;
        let typevar_kwargs = [("covariant", &true.into_bound_py_any(py)?), ("bound", &int)].into_py_dict(py)?;

        m.add("Uint32", &int)?;
        m.add("Uint64", &int)?;
        m.add("Uint128", &int)?;
        m.add("T_co", typevar.call(("T_co",), Some(&typevar_kwargs))?)?;
        m.add("runtime", pyo3::Py::new(py, super::TokioRuntime::new()?)?)
    }
}
