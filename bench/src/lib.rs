use pyo3::Bound;
use pyo3::Py;
use pyo3::PyAny;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::pyclass;
use pyo3::pymethods;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyBytes;
use pyo3::types::PyBytesMethods;
use pyo3::types::PyDict;

#[cfg_attr(Py_3_10, pyclass(name = "compat32", frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(name = "compat32", frozen))]
struct Compat32 {
    hash_function: Py<PyAny>,
    kwargs: Option<Py<PyDict>>,
}

#[cfg_attr(Py_3_10, pyclass(name = "compat64", frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(name = "compat64", frozen))]
struct Compat64 {
    hash_function: Py<PyAny>,
    kwargs: Option<Py<PyDict>>,
}

#[cfg_attr(Py_3_10, pyclass(name = "compat128", frozen, immutable_type))]
#[cfg_attr(not(Py_3_10), pyclass(name = "compat128", frozen))]
struct Compat128 {
    hash_function: Py<PyAny>,
    kwargs: Option<Py<PyDict>>,
}

fn digest<'py>(
    py: Python<'py>,
    hash_function: &Py<PyAny>,
    kwargs: Option<&Py<PyDict>>,
    data: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyAny>> {
    hash_function
        .bind(py)
        .call((data,), kwargs.as_ref().map(|k| k.bind(py)))?
        .call_method0(pyo3::intern!(py, "digest"))
}

#[pymethods]
impl Compat32 {
    #[new]
    #[pyo3(signature = (hash_function, /, **kwargs))]
    fn new(hash_function: Py<PyAny>, kwargs: Option<Bound<'_, PyDict>>) -> Self {
        Self {
            hash_function,
            kwargs: kwargs.map(Bound::unbind),
        }
    }

    #[pyo3(signature = (data, /))]
    fn __call__(&self, py: Python<'_>, data: Bound<'_, PyAny>) -> PyResult<u32> {
        let digest = digest(py, &self.hash_function, self.kwargs.as_ref(), data)?;
        Ok(u32::from_le_bytes(digest.cast::<PyBytes>()?.as_bytes().try_into()?))
    }
}

#[pymethods]
impl Compat64 {
    #[new]
    #[pyo3(signature = (hash_function, /, **kwargs))]
    fn new(hash_function: Py<PyAny>, kwargs: Option<Bound<'_, PyDict>>) -> Self {
        Self {
            hash_function,
            kwargs: kwargs.map(Bound::unbind),
        }
    }

    #[pyo3(signature = (data, /))]
    fn __call__(&self, py: Python<'_>, data: Bound<'_, PyAny>) -> PyResult<u64> {
        let digest = digest(py, &self.hash_function, self.kwargs.as_ref(), data)?;
        Ok(u64::from_le_bytes(digest.cast::<PyBytes>()?.as_bytes().try_into()?))
    }
}

#[pymethods]
impl Compat128 {
    #[new]
    #[pyo3(signature = (hash_function, /, **kwargs))]
    fn new(hash_function: Py<PyAny>, kwargs: Option<Bound<'_, PyDict>>) -> Self {
        Self {
            hash_function,
            kwargs: kwargs.map(Bound::unbind),
        }
    }

    #[pyo3(signature = (data, /))]
    fn __call__(&self, py: Python<'_>, data: Bound<'_, PyAny>) -> PyResult<u128> {
        let digest = digest(py, &self.hash_function, self.kwargs.as_ref(), data)?;
        Ok(u128::from_le_bytes(digest.cast::<PyBytes>()?.as_bytes().try_into()?))
    }
}

#[pyo3::pymodule(name = "compat", gil_used = false)]
mod compat_module {
    #[pymodule_export]
    use super::Compat32;
    #[pymodule_export]
    use super::Compat64;
    #[pymodule_export]
    use super::Compat128;
}
