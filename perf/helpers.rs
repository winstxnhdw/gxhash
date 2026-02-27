use pyo3::Bound;
use pyo3::PyAny;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyModule;

pub static ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Clone, Copy)]
pub enum Memory {
    B64 = 64,
    KiB64 = 64 << 10,
    MiB4 = 4 << 20,
}

impl From<Memory> for usize {
    fn from(m: Memory) -> usize {
        m as usize
    }
}

pub fn generate_bytes(seed: u64, output_size: impl Into<usize>) -> Vec<u8> {
    let output_size = output_size.into();
    let mut state = seed.wrapping_add(1);
    let mut out = Vec::with_capacity(output_size);

    while out.len() < output_size {
        state = state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
        out.extend_from_slice(&state.to_le_bytes());
    }

    out.truncate(output_size);
    out
}

macro_rules! python {
    ($py:ident, $body:block) => {{
        helpers::ONCE.call_once(|| {
            pyo3::append_to_inittab!(gxhash_py);
            pyo3::Python::initialize();
        });

        let result = pyo3::Python::attach(|$py| -> pyo3::PyResult<()> {
            $body
            Ok(())
        });

        result.expect("Something went wrong with Python!")
    }};
}

pub trait PythonExt<'py> {
    fn import_asyncio(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash32(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhash64(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhash128(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhashlib32(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhashlib64(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhashlib128(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhashlib_new(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhashlib_file_digest(&self) -> PyResult<Bound<'_, PyAny>>;
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

    fn import_gxhash32(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.core"))?
            .getattr(intern!(*self, "GxHash32"))
    }

    fn import_gxhash64(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.core"))?
            .getattr(intern!(*self, "GxHash64"))
    }

    fn import_gxhash128(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.core"))?
            .getattr(intern!(*self, "GxHash128"))
    }

    fn import_gxhashlib32(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.gxhashlib"))?
            .getattr(intern!(*self, "gxhash32"))
    }

    fn import_gxhashlib64(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.gxhashlib"))?
            .getattr(intern!(*self, "gxhash64"))
    }

    fn import_gxhashlib128(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.gxhashlib"))?
            .getattr(intern!(*self, "gxhash128"))
    }

    fn import_gxhashlib_new(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.gxhashlib"))?
            .getattr(intern!(*self, "new"))
    }

    fn import_gxhashlib_file_digest(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.gxhashlib"))?
            .getattr(intern!(*self, "file_digest"))
    }
}
