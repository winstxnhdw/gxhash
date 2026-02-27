#[macro_use]
mod helpers;

use divan::Bencher;
use helpers::generate_bytes;
use helpers::Memory;
use helpers::PythonExt;

use pyo3::types::PyAnyMethods;
use pyo3::types::PyTuple;

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
                    let coroutines: Vec<_> = payloads
                        .iter()
                        .flat_map(|bytes| hash_async.call1((bytes.as_slice(),)))
                        .collect();

                    run_until_complete.call1((asyncio_gather.call1(PyTuple::new(py, &coroutines)?)?,))
                });
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

fn main() {
    divan::main()
}
