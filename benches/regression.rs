use divan::Bencher;
use gxhash::gxhash_py;

use pyo3::Bound;
use pyo3::IntoPyObject;
use pyo3::PyAny;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyListMethods;
use pyo3::types::PyModule;

static BYTES: [u8; 1024] = [0u8; 1024];
static ONCE: std::sync::Once = std::sync::Once::new();

macro_rules! python {
    ($py:ident, $body:block) => {{
        ONCE.call_once(|| {
            pyo3::append_to_inittab!(gxhash_py);
            pyo3::Python::initialize();
        });

        pyo3::Python::attach(|$py| -> pyo3::PyResult<()> {
            $body
            Ok(())
        }).expect("Something went wrong with Python!")
    }};
}

trait PythonExt<'py> {
    fn import_asyncio(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash(&self) -> PyResult<Bound<'_, PyAny>>;
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

    fn import_gxhash(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash"))?
            .getattr(intern!(*self, "GxHash128"))
    }
}

#[divan::bench]
fn hash(bencher: Bencher) {
    python!(py, {
        let seed = 42;
        let hasher = py.import_gxhash()?.call1((seed,))?;

        bencher.bench_local(|| hasher.call_method1("hash", (BYTES,)));
    })
}

#[divan::bench]
fn hash_async(bencher: Bencher) {
    python!(py, {
        let seed = 42;
        let asyncio = py.import_asyncio()?;
        let hash_async = py.import_gxhash()?.call1((seed,))?.getattr("hash_async")?;
        let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
        let run_until_complete = asyncio_loop.getattr("run_until_complete")?;

        asyncio.call_method1("set_event_loop", (&asyncio_loop,))?;
        bencher.bench_local(|| run_until_complete.call1((hash_async.call1((BYTES,))?,)));
    })
}

#[divan::bench]
fn hash_async_batch(bencher: Bencher) {
    python!(py, {
        let seed = 42;
        let payloads = vec![BYTES; 100];
        let asyncio = py.import_asyncio()?;
        let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
        let asyncio_gather = asyncio.getattr("gather")?;
        let run_until_complete = asyncio_loop.getattr("run_until_complete")?;
        let hash_async = py.import_gxhash()?.call1((seed,))?.getattr("hash_async")?;

        asyncio.call_method1("set_event_loop", (&asyncio_loop,))?;
        bencher.bench_local(|| {
            let coroutines = payloads
                .iter()
                .flat_map(|bytes| hash_async.call1((bytes,)))
                .collect::<Vec<_>>()
                .into_pyobject(py)?
                .cast::<pyo3::types::PyList>()?
                .to_tuple();

            run_until_complete.call1((&asyncio_gather.call1(&coroutines)?,))
        });
    })
}

fn main() {
    divan::main()
}
