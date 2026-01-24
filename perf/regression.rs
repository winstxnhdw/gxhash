use divan::Bencher;
use gxhash::gxhash_py;

use pyo3::Bound;
use pyo3::IntoPyObjectExt;
use pyo3::PyAny;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyListMethods;
use pyo3::types::PyModule;

static ONCE: std::sync::Once = std::sync::Once::new();

fn generate_bytes(seed: u64) -> Vec<u8> {
    let mut state = seed.wrapping_add(1);
    let mut out = Vec::with_capacity(65536);

    for _ in 0..(65536 / 8) {
        state = state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
        out.extend_from_slice(&state.to_le_bytes());
    }

    out
}

macro_rules! python {
    ($py:ident, $body:block) => {{
        ONCE.call_once(|| {
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

trait PythonExt<'py> {
    fn import_asyncio(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hashlib_gxhash(&self) -> PyResult<Bound<'_, PyAny>>;
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

    fn import_hashlib_gxhash(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import(intern!(*self, "gxhash.hashlib"))?
            .getattr(intern!(*self, "gxhash128"))
    }
}

#[divan::bench]
fn hash(bencher: Bencher) {
    python!(py, {
        let seed: u64 = 42;
        let bytes_vector = generate_bytes(seed);
        let bytes = bytes_vector.as_slice();
        let hash = py.import_gxhash()?.call1((seed,))?.getattr("hash")?;

        bencher.bench_local(|| hash.call1((bytes,)));
    })
}

#[divan::bench]
fn hash_async(bencher: Bencher) {
    python!(py, {
        let seed: u64 = 42;
        let bytes_vector = generate_bytes(seed);
        let bytes = bytes_vector.as_slice();
        let asyncio = py.import_asyncio()?;
        let hash_async = py.import_gxhash()?.call1((seed,))?.getattr("hash_async")?;
        let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
        let run_until_complete = asyncio_loop.getattr("run_until_complete")?;

        asyncio.call_method1("set_event_loop", (&asyncio_loop,))?;
        bencher.bench_local(|| run_until_complete.call1((hash_async.call1((bytes,))?,)));
    })
}

#[divan::bench]
fn hash_async_batch(bencher: Bencher) {
    python!(py, {
        let seed: u64 = 42;
        let payloads = (0..24)
            .map(|i| generate_bytes(seed.wrapping_add(i as u64)))
            .collect::<Vec<_>>();

        let asyncio = py.import_asyncio()?;
        let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
        let asyncio_gather = asyncio.getattr("gather")?;
        let run_until_complete = asyncio_loop.getattr("run_until_complete")?;
        let hash_async = py.import_gxhash()?.call1((seed,))?.getattr("hash_async")?;

        asyncio.call_method1("set_event_loop", (&asyncio_loop,))?;
        bencher.bench_local(|| {
            let coroutines = payloads
                .iter()
                .flat_map(|bytes| hash_async.call1((bytes.as_slice(),)))
                .collect::<Vec<_>>()
                .into_bound_py_any(py)?
                .cast::<pyo3::types::PyList>()?
                .to_tuple();

            run_until_complete.call1((asyncio_gather.call1(coroutines)?,))
        });
    })
}

#[divan::bench]
fn hashlib_gxhash_hexdigest(bencher: Bencher) {
    python!(py, {
        let seed = 42;
        let kwargs = [("seed", seed)].into_py_dict(py)?;
        let hexdigest = py
            .import_hashlib_gxhash()?
            .call((generate_bytes(seed).as_slice(),), Some(&kwargs))?
            .getattr("hexdigest")?;

        bencher.bench_local(|| hexdigest.call0());
    })
}

#[divan::bench]
fn hashlib_gxhash_update(bencher: Bencher) {
    python!(py, {
        let seed = 42;
        let bytes_vector = generate_bytes(seed);
        let bytes = bytes_vector.as_slice();
        let kwargs = [("seed", seed)].into_py_dict(py)?;
        let update = py.import_hashlib_gxhash()?.call((), Some(&kwargs))?.getattr("update")?;

        bencher.bench_local(|| update.call1((bytes,)));
    })
}

fn main() {
    divan::main()
}
