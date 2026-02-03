use crate::helpers::PythonExt;
use crate::helpers::call_hash;
use crate::helpers::call_hash_async;
use crate::pytest;
use gxhash::gxhash_py;
use pyo3::PyResult;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;
use quickcheck_macros::quickcheck;

#[test]
fn test_import_gxhash() -> PyResult<()> {
    pytest!(py, {
        assert!(py.import_gxhash()?.is_instance_of::<pyo3::types::PyModule>())
    })
}

#[test]
fn test_import_gxhash32_from_gxhash() -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash32()?.call1((seed,))?;

        assert!(hasher_class.is_instance_of::<gxhash::GxHash32>())
    })
}

#[test]
fn test_import_gxhash64_from_gxhash() -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash64()?.call1((seed,))?;

        assert!(hasher_class.is_instance_of::<gxhash::GxHash64>())
    })
}

#[test]
fn test_import_gxhash128_from_gxhash() -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash128()?.call1((seed,))?;

        assert!(hasher_class.is_instance_of::<gxhash::GxHash128>())
    })
}

#[test]
fn test_gxhash_async_error() -> PyResult<()> {
    pytest!(py, {
        let gxhash_async_error = py.import_gxhash()?.getattr("GxHashAsyncError")?.call0()?;
        assert!(gxhash_async_error.is_instance_of::<pyo3::exceptions::PyException>())
    })
}

#[quickcheck]
fn test_gxhash32_hash(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash32()?.call((), Some(&seed))?;

        let result1 = call_hash::<u32>(py, &hasher, &bytes)?;
        let result2 = call_hash::<u32>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash64()?.call((), Some(&seed))?;

        let result1 = call_hash::<u64>(py, &hasher, &bytes)?;
        let result2 = call_hash::<u64>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash128()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash::<u128>(py, &hasher, &bytes)?;
        let result2 = call_hash::<u128>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_async(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash32()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash_async::<u32>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u32>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_async(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash64()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash_async::<u64>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u64>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_async(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash128()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash_async::<u128>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u128>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_sync_async_equality(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash32()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash::<u32>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u32>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_sync_async_equality(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash64()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash::<u64>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u64>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_sync_async_equality(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_kwargs = [("seed", seed)].into_py_dict(py)?;
        let hasher = py.import_gxhash128()?.call((), Some(&hasher_kwargs))?;

        let result1 = call_hash::<u128>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u128>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_seed_change(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_class = py.import_gxhash32()?;

        let result1 = call_hash::<u32>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash::<u32>(py, &hasher_class.call1((seed.wrapping_add(1),))?, &bytes)?;
        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_seed_change(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_class = py.import_gxhash64()?;
        let result1 = call_hash::<u64>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash::<u64>(py, &hasher_class.call1((seed.wrapping_add(1),))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_seed_change(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_class = py.import_gxhash128()?;
        let result1 = call_hash::<u128>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash::<u128>(py, &hasher_class.call1((seed.wrapping_add(1),))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_async_seed_change(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_class = py.import_gxhash32()?;
        let result1 = call_hash_async::<u32>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash_async::<u32>(py, &hasher_class.call1((seed.wrapping_add(1),))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_async_seed_change(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_class = py.import_gxhash64()?;
        let result1 = call_hash_async::<u64>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash_async::<u64>(py, &hasher_class.call1((seed.wrapping_add(1),))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_async_seed_change(seed: i64, bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher_class = py.import_gxhash128()?;
        let result1 = call_hash_async::<u128>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash_async::<u128>(py, &hasher_class.call1((seed.wrapping_add(1),))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_async_large_input(seed: i64) -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash32()?.call1((seed,))?;
        let input = vec![0u8; 16 << 20];

        let result1 = call_hash_async::<u32>(py, &hasher, &input)?;
        let result2 = call_hash::<u32>(py, &hasher, &input)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_async_large_input(seed: i64) -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash64()?.call1((seed,))?;
        let input = vec![0u8; 16 << 20];

        let result1 = call_hash_async::<u64>(py, &hasher, &input)?;
        let result2 = call_hash::<u64>(py, &hasher, &input)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_async_large_input(seed: i64) -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash128()?.call1((seed,))?;
        let input = vec![0u8; 16 << 20];

        let result1 = call_hash_async::<u128>(py, &hasher, &input)?;
        let result2 = call_hash::<u128>(py, &hasher, &input)?;

        assert_eq!(result1, result2);
    })
}

#[test]
fn test_gxhash32_hash_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash32()?.call1((42,))?;
        let result = call_hash::<u32>(py, &hasher, b"hello")?;

        assert_eq!(result, 11074207u32);
    })
}

#[test]
fn test_gxhash64_hash_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash64()?.call1((42,))?;
        let result = call_hash::<u64>(py, &hasher, b"hello")?;

        assert_eq!(result, 10922345113571621535u64);
    })
}

#[test]
fn test_gxhash128_hash_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash128()?.call1((42,))?;
        let result = call_hash::<u128>(py, &hasher, b"hello")?;

        assert_eq!(result, 340008176428847722652273161291189254815u128);
    })
}

#[test]
fn test_gxhash32_hash_async_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash32()?.call1((42,))?;
        let result = call_hash_async::<u32>(py, &hasher, b"hello")?;

        assert_eq!(result, 11074207u32);
    })
}

#[test]
fn test_gxhash64_hash_async_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash64()?.call1((42,))?;
        let result = call_hash_async::<u64>(py, &hasher, b"hello")?;

        assert_eq!(result, 10922345113571621535u64);
    })
}

#[test]
fn test_gxhash128_hash_async_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_gxhash128()?.call1((42,))?;
        let result = call_hash_async::<u128>(py, &hasher, b"hello")?;

        assert_eq!(result, 340008176428847722652273161291189254815u128);
    })
}
