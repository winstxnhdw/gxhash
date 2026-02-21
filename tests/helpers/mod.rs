use pyo3::Bound;
use pyo3::PyAny;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyBytes;
use pyo3::types::PyMemoryView;
use pyo3::types::PyModule;

pub(crate) static ONCE: std::sync::Once = std::sync::Once::new();

#[macro_export]
macro_rules! pytest {
    ($py:ident, $body:block) => {{
        $crate::helpers::ONCE.call_once(|| {
            pyo3::append_to_inittab!(gxhash_py);
            pyo3::Python::initialize();
        });

        pyo3::Python::attach(|$py| -> pyo3::PyResult<()> {
            $body
            Ok(())
        })
    }};
}

pub trait PythonExt<'py> {
    fn import_asyncio(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash_hashlib(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash32(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhash64(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhash128(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hashlib_gxhash32(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hashlib_gxhash64(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hashlib_gxhash128(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hashlib_new(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hashlib_file_digest(&self) -> PyResult<Bound<'_, PyAny>>;
}

impl<'py> PythonExt<'py> for Python<'py> {
    #[inline(always)]
    fn import_asyncio(&self) -> PyResult<Bound<'_, PyModule>> {
        let asyncio = self.import(intern!(*self, "asyncio"))?;

        #[cfg(windows)]
        {
            let policy = asyncio.getattr(intern!(*self, "WindowsSelectorEventLoopPolicy"))?;
            asyncio.call_method1(intern!(*self, "set_event_loop_policy"), (policy.call0()?,))?;
        }

        Ok(asyncio)
    }

    fn import_gxhash(&self) -> PyResult<Bound<'_, PyModule>> {
        self.import(intern!(*self, "gxhash.core"))
    }

    fn import_gxhash32(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "GxHash32"))
    }

    fn import_gxhash64(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "GxHash64"))
    }

    fn import_gxhash128(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "GxHash128"))
    }

    fn import_gxhash_hashlib(&self) -> PyResult<Bound<'_, PyModule>> {
        self.import(intern!(*self, "gxhash.gxhashlib"))
    }

    fn import_hashlib_gxhash32(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash_hashlib()?.getattr(intern!(*self, "gxhash32"))
    }

    fn import_hashlib_gxhash64(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash_hashlib()?.getattr(intern!(*self, "gxhash64"))
    }

    fn import_hashlib_gxhash128(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash_hashlib()?.getattr(intern!(*self, "gxhash128"))
    }

    fn import_hashlib_new(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash_hashlib()?.getattr(intern!(*self, "new"))
    }

    fn import_hashlib_file_digest(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash_hashlib()?.getattr(intern!(*self, "file_digest"))
    }
}

pub fn call_hashlib_digest<'py>(py: Python<'py>, hasher: &Bound<'py, PyAny>, bytes: &[u8]) -> PyResult<Vec<u8>> {
    let memoryview = PyMemoryView::from(&PyBytes::new(py, bytes))?;

    hasher
        .call1((&memoryview,))?
        .call_method0(intern!(py, "digest"))?
        .extract()
}

pub fn call_hashlib_hexdigest<'py>(py: Python<'py>, hasher: &Bound<'py, PyAny>, bytes: &[u8]) -> PyResult<String> {
    let memoryview = PyMemoryView::from(&PyBytes::new(py, bytes))?;

    hasher
        .call1((&memoryview,))?
        .call_method0(intern!(py, "hexdigest"))?
        .extract()
}

pub fn call_hash<'py, T>(py: Python<'py>, obj: &Bound<'py, PyAny>, bytes: &[u8]) -> PyResult<T>
where
    for<'s> T: pyo3::FromPyObject<'s, 's, Error = pyo3::PyErr>,
{
    obj.call_method1(intern!(py, "hash"), (bytes,))?.extract()
}

pub fn call_hash_async<'py, T>(py: Python<'py>, obj: &Bound<'py, PyAny>, bytes: &[u8]) -> PyResult<T>
where
    for<'s> T: pyo3::FromPyObject<'s, 's, Error = pyo3::PyErr>,
{
    py.import_asyncio()?
        .getattr(intern!(py, "run"))?
        .call1((obj.call_method1(intern!(py, "hash_async"), (bytes,))?,))?
        .extract()
}
