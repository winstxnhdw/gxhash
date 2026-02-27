#[macro_use]
mod helpers;

use divan::Bencher;
use gxhash::gxhash_py;
use helpers::generate_bytes;
use helpers::Memory;
use helpers::PythonExt;

use pyo3::intern;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;

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
