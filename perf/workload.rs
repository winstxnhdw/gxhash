#[macro_use]
mod helpers;

use divan::Bencher;
use gxhash::gxhash_py;
use helpers::generate_bytes;
use helpers::PythonExt;

use pyo3::types::PyAnyMethods;
use pyo3::types::PyBytes;
use pyo3::types::PyTuple;

struct PseudoRNG {
    state: u64,
}

impl PseudoRNG {
    fn new(seed: u64) -> Self {
        Self {
            state: seed.wrapping_add(1),
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn range(&mut self, min: u64, max: u64) -> u64 {
        min + self.next_u64() % (max - min + 1)
    }
}

fn random_batch_size(rng: &mut PseudoRNG) -> usize {
    match rng.range(0, 99) {
        0..40 => 1,
        40..70 => rng.range(2, 4) as usize,
        70..90 => rng.range(5, 12) as usize,
        _ => rng.range(13, 32) as usize,
    }
}

fn random_delay(rng: &mut PseudoRNG) -> u128 {
    (match rng.range(0, 99) {
        0..20 => 0,
        20..50 => rng.range(1_000, 50_000),
        50..80 => rng.range(50_000, 200_000),
        80..95 => rng.range(200_000, 500_000),
        _ => rng.range(500_000, 2_000_000),
    }) as u128
}

fn random_payload_size(rng: &mut PseudoRNG) -> usize {
    match rng.range(0, 99) {
        0..50 => rng.range(32, 256) as usize,
        50..75 => rng.range(256, 8192) as usize,
        75..90 => rng.range(8192, 262144) as usize,
        90..98 => rng.range(262144, 2097152) as usize,
        _ => rng.range(2097152, 8388608) as usize,
    }
}

fn generate_batch<'py>(
    py: pyo3::Python<'py>,
    rng: &mut PseudoRNG,
    batch_size: usize,
) -> Vec<pyo3::Bound<'py, PyBytes>> {
    (0..batch_size)
        .map(|_| PyBytes::new(py, &generate_bytes(rng.next_u64(), random_payload_size(rng))))
        .collect()
}

#[divan::bench]
fn workload_simulation(bencher: Bencher) {
    python!(py, {
        let seed = 42u64;
        let payload_batches_size = 200;
        let warmup_payload = PyBytes::new(py, &vec![0u8; 32 << 20]);
        let mut rng = PseudoRNG::new(seed);
        let batch_sizes: Vec<_> = (0..payload_batches_size).map(|_| random_batch_size(&mut rng)).collect();
        let delays: Vec<_> = (0..payload_batches_size).map(|_| random_delay(&mut rng)).collect();
        let payload_batches: Vec<Vec<_>> = batch_sizes
            .iter()
            .map(|&batch_size| generate_batch(py, &mut rng, batch_size))
            .collect();

        let hash_async = py.import_gxhash64()?.call1((seed,))?.getattr("hash_async")?;
        let asyncio = py.import_asyncio()?;
        let asyncio_loop = asyncio.getattr("new_event_loop")?.call0()?;
        let asyncio_gather = asyncio.getattr("gather")?;
        let run_until_complete = asyncio_loop.getattr("run_until_complete")?;
        asyncio.call_method1("set_event_loop", (&asyncio_loop,))?;

        for _ in 0..50 {
            run_until_complete.call1((hash_async.call1((&warmup_payload,))?,))?;
        }

        bencher.bench_local(|| -> pyo3::PyResult<()> {
            for (payloads, &delay) in payload_batches.iter().zip(&delays) {
                let coroutines: Vec<_> = payloads
                    .iter()
                    .flat_map(|payload| hash_async.call1((payload,)))
                    .collect();

                run_until_complete.call1((asyncio_gather.call1(PyTuple::new(py, &coroutines)?)?,))?;
                std::thread::sleep(std::time::Duration::from_nanos_u128(delay));
            }

            Ok(())
        });
    });
}

fn main() {
    divan::main()
}
