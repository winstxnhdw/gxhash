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

#[derive(Clone, Copy)]
enum Memory {
    B64 = 64,
    KiB64 = 64 << 10,
    MiB4 = 4 << 20,
}

fn generate_bytes(seed: u64, output_size: Memory) -> Vec<u8> {
    let mut state = seed.wrapping_add(1);
    let mut out = Vec::with_capacity(output_size as usize);

    while out.len() < output_size as usize {
        state = state.wrapping_mul(6364136223846793005u64).wrapping_add(1);
        out.extend_from_slice(&state.to_le_bytes());
    }

    out.truncate(output_size as usize);
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

macro_rules! bench_hash {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed: u64 = 42;
                let bytes_vector = generate_bytes(seed, $memory);
                let bytes = bytes_vector.as_slice();
                let hash = py.$import()?.call1((seed,))?.getattr("hash")?;

                bencher.bench_local(|| hash.call1((bytes,)));
            })
        }
    };
}

macro_rules! bench_hash_async {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed: u64 = 42;
                let bytes_vector = generate_bytes(seed, $memory);
                let bytes = bytes_vector.as_slice();
                let asyncio = py.import_asyncio()?;
                let hash_async = py.$import()?.call1((seed,))?.getattr("hash_async")?;
                let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
                let run_until_complete = asyncio_loop.getattr("run_until_complete")?;

                asyncio.call_method1("set_event_loop", (&asyncio_loop,))?;
                bencher.bench_local(|| run_until_complete.call1((hash_async.call1((bytes,))?,)));
            })
        }
    };
}

macro_rules! bench_hash_async_batch {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed: u64 = 42;
                let payloads = (0..24)
                    .map(|i| generate_bytes(seed.wrapping_add(i as u64), $memory))
                    .collect::<Vec<_>>();

                let asyncio = py.import_asyncio()?;
                let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
                let asyncio_gather = asyncio.getattr("gather")?;
                let run_until_complete = asyncio_loop.getattr("run_until_complete")?;
                let hash_async = py.$import()?.call1((seed,))?.getattr("hash_async")?;

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
    };
}

macro_rules! bench_hashlib_digest {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed = 42;
                let kwargs = [("seed", seed)].into_py_dict(py)?;
                let digest = py
                    .$import()?
                    .call((generate_bytes(seed, $memory).as_slice(),), Some(&kwargs))?
                    .getattr("digest")?;

                bencher.bench_local(|| digest.call0());
            })
        }
    };
}

macro_rules! bench_hashlib_hexdigest {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed = 42;
                let kwargs = [("seed", seed)].into_py_dict(py)?;
                let hexdigest = py
                    .$import()?
                    .call((generate_bytes(seed, $memory).as_slice(),), Some(&kwargs))?
                    .getattr("hexdigest")?;

                bencher.bench_local(|| hexdigest.call0());
            })
        }
    };
}

macro_rules! bench_hashlib_update {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed = 42;
                let bytes_vector = generate_bytes(seed, $memory);
                let bytes = bytes_vector.as_slice();
                let kwargs = [("seed", seed)].into_py_dict(py)?;
                let update = py.$import()?.call((), Some(&kwargs))?.getattr("update")?;

                bencher.bench_local(|| update.call1((bytes,)));
            })
        }
    };
}

macro_rules! bench_hashlib_copy {
    ($name:ident, $import:ident, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed = 42;
                let kwargs = [("seed", seed)].into_py_dict(py)?;
                let hasher = py.$import()?.call((), Some(&kwargs))?;
                let copy = hasher.getattr("copy")?;

                hasher.call_method1("update", (generate_bytes(seed, $memory),))?;
                bencher.bench_local(|| copy.call0());
            })
        }
    };
}

macro_rules! bench_hashlib_new {
    ($name:ident, $algo:literal, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed = 42;
                let bytes_vector = generate_bytes(seed as u64, $memory);
                let bytes = bytes_vector.as_slice();
                let new = py.import_gxhashlib_new()?;
                let kwargs = [("seed", seed)].into_py_dict(py)?;

                bencher.bench_local(|| new.call(($algo, bytes), Some(&kwargs)));
            })
        }
    };
}

macro_rules! bench_hashlib_file_digest {
    ($name:ident, $algo:literal, $memory:expr) => {
        #[divan::bench]
        fn $name(bencher: Bencher) {
            python!(py, {
                let seed = 42;
                let bytes_vector = generate_bytes(seed as u64, $memory);
                let io = py.import(intern!(py, "io"))?;
                let file_digest = py.import_gxhashlib_file_digest()?;
                let kwargs = [("seed", seed)].into_py_dict(py)?;
                let buf = io.call_method1(intern!(py, "BytesIO"), (bytes_vector.as_slice(),))?;

                bencher.bench_local(|| file_digest.call((&buf, $algo), Some(&kwargs)));
            })
        }
    };
}

bench_hash!(hash32_small, import_gxhash32, Memory::B64);
bench_hash!(hash32, import_gxhash32, Memory::KiB64);
bench_hash!(hash64_small, import_gxhash64, Memory::B64);
bench_hash!(hash64, import_gxhash64, Memory::KiB64);
bench_hash!(hash128_small, import_gxhash128, Memory::B64);
bench_hash!(hash128, import_gxhash128, Memory::KiB64);

bench_hash_async!(hash32_async_small, import_gxhash32, Memory::B64);
bench_hash_async!(hash32_async, import_gxhash32, Memory::KiB64);
bench_hash_async!(hash64_async_small, import_gxhash64, Memory::B64);
bench_hash_async!(hash64_async, import_gxhash64, Memory::KiB64);
bench_hash_async!(hash128_async_small, import_gxhash128, Memory::B64);
bench_hash_async!(hash128_async, import_gxhash128, Memory::KiB64);

bench_hash_async_batch!(hash32_async_batch_small, import_gxhash32, Memory::B64);
bench_hash_async_batch!(hash32_async_batch, import_gxhash32, Memory::KiB64);
bench_hash_async_batch!(hash32_async_batch_large, import_gxhash32, Memory::MiB4);
bench_hash_async_batch!(hash64_async_batch_small, import_gxhash64, Memory::B64);
bench_hash_async_batch!(hash64_async_batch, import_gxhash64, Memory::KiB64);
bench_hash_async_batch!(hash64_async_batch_large, import_gxhash64, Memory::MiB4);
bench_hash_async_batch!(hash128_async_batch_small, import_gxhash128, Memory::B64);
bench_hash_async_batch!(hash128_async_batch, import_gxhash128, Memory::KiB64);
bench_hash_async_batch!(hash128_async_batch_large, import_gxhash128, Memory::MiB4);

bench_hashlib_digest!(gxhashlib32_digest_small, import_gxhashlib32, Memory::B64);
bench_hashlib_digest!(gxhashlib32_digest, import_gxhashlib32, Memory::KiB64);
bench_hashlib_digest!(gxhashlib64_digest_small, import_gxhashlib64, Memory::B64);
bench_hashlib_digest!(gxhashlib64_digest, import_gxhashlib64, Memory::KiB64);
bench_hashlib_digest!(gxhashlib128_digest_small, import_gxhashlib128, Memory::B64);
bench_hashlib_digest!(gxhashlib128_digest, import_gxhashlib128, Memory::KiB64);

bench_hashlib_hexdigest!(gxhashlib32_hexdigest_small, import_gxhashlib32, Memory::B64);
bench_hashlib_hexdigest!(gxhashlib32_hexdigest, import_gxhashlib32, Memory::KiB64);
bench_hashlib_hexdigest!(gxhashlib64_hexdigest_small, import_gxhashlib64, Memory::B64);
bench_hashlib_hexdigest!(gxhashlib64_hexdigest, import_gxhashlib64, Memory::KiB64);
bench_hashlib_hexdigest!(gxhashlib128_hexdigest_small, import_gxhashlib128, Memory::B64);
bench_hashlib_hexdigest!(gxhashlib128_hexdigest, import_gxhashlib128, Memory::KiB64);

bench_hashlib_update!(gxhashlib32_update_small, import_gxhashlib32, Memory::B64);
bench_hashlib_update!(gxhashlib32_update, import_gxhashlib32, Memory::KiB64);
bench_hashlib_update!(gxhashlib64_update_small, import_gxhashlib64, Memory::B64);
bench_hashlib_update!(gxhashlib64_update, import_gxhashlib64, Memory::KiB64);
bench_hashlib_update!(gxhashlib128_update_small, import_gxhashlib128, Memory::B64);
bench_hashlib_update!(gxhashlib128_update, import_gxhashlib128, Memory::KiB64);

bench_hashlib_copy!(gxhashlib32_copy, import_gxhashlib32, Memory::KiB64);
bench_hashlib_copy!(gxhashlib64_copy, import_gxhashlib64, Memory::KiB64);
bench_hashlib_copy!(gxhashlib128_copy, import_gxhashlib128, Memory::KiB64);

bench_hashlib_new!(gxhashlib_new32_small, "gxhash32", Memory::B64);
bench_hashlib_new!(gxhashlib_new32, "gxhash32", Memory::KiB64);
bench_hashlib_new!(gxhashlib_new64_small, "gxhash64", Memory::B64);
bench_hashlib_new!(gxhashlib_new64, "gxhash64", Memory::KiB64);
bench_hashlib_new!(gxhashlib_new128_small, "gxhash128", Memory::B64);
bench_hashlib_new!(gxhashlib_new128, "gxhash128", Memory::KiB64);

bench_hashlib_file_digest!(gxhashlib_file_digest32_small, "gxhash32", Memory::B64);
bench_hashlib_file_digest!(gxhashlib_file_digest32, "gxhash32", Memory::KiB64);
bench_hashlib_file_digest!(gxhashlib_file_digest64_small, "gxhash64", Memory::B64);
bench_hashlib_file_digest!(gxhashlib_file_digest64, "gxhash64", Memory::KiB64);
bench_hashlib_file_digest!(gxhashlib_file_digest128_small, "gxhash128", Memory::B64);
bench_hashlib_file_digest!(gxhashlib_file_digest128, "gxhash128", Memory::KiB64);

fn main() {
    divan::main()
}
