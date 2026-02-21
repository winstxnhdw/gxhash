use crate::helpers::{PythonExt, call_hashlib_digest, call_hashlib_hexdigest};
use crate::pytest;
use gxhash::gxhash_py;
use pyo3::PyResult;
use pyo3::intern;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyType;
use pyo3::types::PyTypeMethods;
use quickcheck_macros::quickcheck;

#[test]
fn test_import_gxhash_hashlib() -> PyResult<()> {
    pytest!(py, {
        assert!(py.import_gxhash_hashlib()?.is_instance_of::<pyo3::types::PyModule>())
    })
}

#[test]
fn test_hashlib_gxhash32_name() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "name"))?.extract::<String>()?, "gxhash32");
    })
}

#[test]
fn test_hashlib_gxhash64_name() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "name"))?.extract::<String>()?, "gxhash64");
    })
}

#[test]
fn test_hashlib_gxhash128_name() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "name"))?.extract::<String>()?, "gxhash128");
    })
}

#[test]
fn test_hashlib_gxhash32_block_size() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "block_size"))?.extract::<usize>()?, 1);
    })
}

#[test]
fn test_hashlib_gxhash64_block_size() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "block_size"))?.extract::<usize>()?, 1);
    })
}

#[test]
fn test_hashlib_gxhash128_block_size() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "block_size"))?.extract::<usize>()?, 1);
    })
}

#[test]
fn test_hashlib_gxhash32_digest_size() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "digest_size"))?.extract::<usize>()?, 4);
    })
}

#[test]
fn test_hashlib_gxhash64_digest_size() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "digest_size"))?.extract::<usize>()?, 8);
    })
}

#[test]
fn test_hashlib_gxhash128_digest_size() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call0()?;
        assert_eq!(hasher.getattr(intern!(py, "digest_size"))?.extract::<usize>()?, 16);
    })
}

#[test]
fn test_hashlib_gxhash32_digest() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call1((b"hello",))?;
        let digest = hasher.call_method0(intern!(py, "digest"))?;
        assert_eq!(digest.extract::<&[u8]>()?.len(), 4);
    })
}

#[test]
fn test_hashlib_gxhash64_digest() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call1((b"hello",))?;
        let digest = hasher.call_method0(intern!(py, "digest"))?;
        assert_eq!(digest.extract::<&[u8]>()?.len(), 8);
    })
}

#[test]
fn test_hashlib_gxhash128_digest() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call1((b"hello",))?;
        let digest = hasher.call_method0(intern!(py, "digest"))?;
        assert_eq!(digest.extract::<&[u8]>()?.len(), 16);
    })
}

#[test]
fn test_hashlib_gxhash32_hexdigest() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call1((b"hello",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?;
        assert_eq!(hexdigest.extract::<String>()?.len(), 8);
    })
}

#[test]
fn test_hashlib_gxhash64_hexdigest() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call1((b"hello",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?;
        assert_eq!(hexdigest.extract::<String>()?.len(), 16);
    })
}

#[test]
fn test_hashlib_gxhash128_hexdigest() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call1((b"hello",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?;
        assert_eq!(hexdigest.extract::<String>()?.len(), 32);
    })
}

#[test]
fn test_hashlib_gxhash32_update() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call1((b"hello ",))?;
        hasher.call_method1(intern!(py, "update"), (b"world",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        let combined_hasher = py.import_hashlib_gxhash32()?.call1((b"hello world",))?;
        let combined_hexdigest = combined_hasher
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(hexdigest, combined_hexdigest);
    })
}

#[test]
fn test_hashlib_gxhash64_update() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call1((b"hello ",))?;
        hasher.call_method1(intern!(py, "update"), (b"world",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let combined_hexdigest = py
            .import_hashlib_gxhash64()?
            .call1((b"hello world",))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(hexdigest, combined_hexdigest);
    })
}

#[test]
fn test_hashlib_gxhash128_update() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call1((b"hello ",))?;
        hasher.call_method1(intern!(py, "update"), (b"world",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let combined_hexdigest = py
            .import_hashlib_gxhash128()?
            .call1((b"hello world",))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(hexdigest, combined_hexdigest);
    })
}

#[test]
fn test_hashlib_gxhash32_copy() -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash32()?.call1((b"hello",))?;
        let hasher2 = hasher1.call_method0(intern!(py, "copy"))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest1, hexdigest2);
    })
}

#[test]
fn test_hashlib_gxhash64_copy() -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash64()?.call1((b"hello",))?;
        let hasher2 = hasher1.call_method0(intern!(py, "copy"))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest1, hexdigest2);
    })
}

#[test]
fn test_hashlib_gxhash128_copy() -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash128()?.call1((b"hello",))?;
        let hasher2 = hasher1.call_method0(intern!(py, "copy"))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest1, hexdigest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash32_hexdigest_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let gxhash32 = py.import_hashlib_gxhash32()?;
        let hexdigest = call_hashlib_hexdigest(py, &gxhash32, &bytes)?;
        let digest: String = call_hashlib_digest(py, &gxhash32, &bytes)?
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();

        assert_eq!(hexdigest, digest);
    })
}

#[quickcheck]
fn test_hashlib_gxhash64_hexdigest_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let gxhash64 = py.import_hashlib_gxhash64()?;
        let hexdigest = call_hashlib_hexdigest(py, &gxhash64, &bytes)?;
        let digest: String = call_hashlib_digest(py, &gxhash64, &bytes)?
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();

        assert_eq!(hexdigest, digest);
    })
}

#[quickcheck]
fn test_hashlib_gxhash128_hexdigest_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let gxhash128 = py.import_hashlib_gxhash128()?;
        let hexdigest = call_hashlib_hexdigest(py, &gxhash128, &bytes)?;
        let digest: String = call_hashlib_digest(py, &gxhash128, &bytes)?
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();

        assert_eq!(hexdigest, digest);
    })
}

#[test]
fn test_hashlib_gxhash32_seed() -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let kwargs1 = [("seed", seed)].into_py_dict(py)?;
        let kwargs2 = [("seed", seed + 1)].into_py_dict(py)?;

        let hasher1 = py.import_hashlib_gxhash32()?.call((b"hello",), Some(&kwargs1))?;
        let hasher2 = py.import_hashlib_gxhash32()?.call((b"hello",), Some(&kwargs2))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_ne!(hexdigest1, hexdigest2);
    })
}

#[test]
fn test_hashlib_gxhash64_seed() -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let kwargs1 = [("seed", seed)].into_py_dict(py)?;
        let kwargs2 = [("seed", seed + 1)].into_py_dict(py)?;

        let hasher1 = py.import_hashlib_gxhash64()?.call((b"hello",), Some(&kwargs1))?;
        let hasher2 = py.import_hashlib_gxhash64()?.call((b"hello",), Some(&kwargs2))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_ne!(hexdigest1, hexdigest2);
    })
}

#[test]
fn test_hashlib_gxhash128_seed() -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let kwargs1 = [("seed", seed)].into_py_dict(py)?;
        let kwargs2 = [("seed", seed + 1)].into_py_dict(py)?;

        let hasher1 = py.import_hashlib_gxhash128()?.call((b"hello",), Some(&kwargs1))?;
        let hasher2 = py.import_hashlib_gxhash128()?.call((b"hello",), Some(&kwargs2))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_ne!(hexdigest1, hexdigest2);
    })
}

#[test]
fn test_hashlib_gxhash32_hexdigest_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call1((b"hello",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest, "9470c7ff");
    })
}

#[test]
fn test_hashlib_gxhash64_hexdigest_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call1((b"hello",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest, "9470c7ff1bf0b0ee");
    })
}

#[test]
fn test_hashlib_gxhash128_hexdigest_determinism() -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call1((b"hello",))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest, "9470c7ff1bf0b0ee455b4d92b9ef3160");
    })
}

#[test]
fn test_hashlib_gxhash32_digest_determinism() -> PyResult<()> {
    pytest!(py, {
        let expected_digest = [148, 112, 199, 255];
        let hasher = py.import_hashlib_gxhash32()?.call1((b"hello",))?;
        let digest = hasher.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;

        assert_eq!(digest, expected_digest);
    })
}

#[test]
fn test_hashlib_gxhash64_digest_determinism() -> PyResult<()> {
    pytest!(py, {
        let expected_digest = [148, 112, 199, 255, 27, 240, 176, 238];
        let hasher = py.import_hashlib_gxhash64()?.call1((b"hello",))?;
        let digest = hasher.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;

        assert_eq!(digest, expected_digest);
    })
}

#[test]
fn test_hashlib_gxhash128_digest_determinism() -> PyResult<()> {
    pytest!(py, {
        let expected_digest = [148, 112, 199, 255, 27, 240, 176, 238, 69, 91, 77, 146, 185, 239, 49, 96];
        let hasher = py.import_hashlib_gxhash128()?.call1((b"hello",))?;
        let digest = hasher.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;

        assert_eq!(digest, expected_digest);
    })
}

#[quickcheck]
fn test_hashlib_new_gxhash32(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let new_digest = py
            .import_hashlib_new()?
            .call1(("gxhash32", bytes.as_slice()))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash32()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(new_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_new_gxhash64(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let new_digest = py
            .import_hashlib_new()?
            .call1(("gxhash64", bytes.as_slice()))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash64()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(new_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_new_gxhash128(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let new_digest = py
            .import_hashlib_new()?
            .call1(("gxhash128", bytes.as_slice()))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash128()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(new_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_new_with_seed(bytes: Vec<u8>, seed: i64) -> PyResult<()> {
    pytest!(py, {
        let kwargs = [("seed", seed)].into_py_dict(py)?;

        let new_digest = py
            .import_hashlib_new()?
            .call(("gxhash64", bytes.as_slice()), Some(&kwargs))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash64()?
            .call((bytes.as_slice(),), Some(&kwargs))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(new_digest, direct_digest);
    })
}

#[test]
fn test_hashlib_new_no_data() -> PyResult<()> {
    pytest!(py, {
        let new_digest = py
            .import_hashlib_new()?
            .call1(("gxhash32",))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash32()?
            .call0()?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(new_digest, direct_digest);
    })
}

#[test]
fn test_hashlib_new_invalid_name() -> PyResult<()> {
    pytest!(py, {
        let error = py.import_hashlib_new()?.call1(("invalid",)).unwrap_err();
        assert!(error.is_instance_of::<pyo3::exceptions::PyValueError>(py));
    })
}

#[quickcheck]
fn test_hashlib_file_digest_io_gxhash32(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let file = py
            .import(intern!(py, "io"))?
            .getattr(intern!(py, "BytesIO"))?
            .call1((bytes.as_slice(),))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call1((&file, "gxhash32"))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash32()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_file_digest_bytesio_gxhash64(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let file = py
            .import(intern!(py, "io"))?
            .getattr(intern!(py, "BytesIO"))?
            .call1((bytes.as_slice(),))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call1((&file, "gxhash64"))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash64()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_file_digest_bytesio_gxhash128(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let file = py
            .import(intern!(py, "io"))?
            .getattr(intern!(py, "BytesIO"))?
            .call1((bytes.as_slice(),))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call1((&file, "gxhash128"))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash128()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_file_digest_bytesio_with_seed(bytes: Vec<u8>, seed: i64) -> PyResult<()> {
    pytest!(py, {
        let kwargs = [("seed", seed)].into_py_dict(py)?;

        let file = py
            .import(intern!(py, "io"))?
            .getattr(intern!(py, "BytesIO"))?
            .call1((bytes.as_slice(),))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call((&file, "gxhash64"), Some(&kwargs))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash64()?
            .call((bytes.as_slice(),), Some(&kwargs))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}

#[quickcheck]
fn test_hashlib_file_digest_real_file(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let file = py
            .import(intern!(py, "tempfile"))?
            .getattr(intern!(py, "NamedTemporaryFile"))?
            .call0()?;

        file.call_method1(intern!(py, "write"), (bytes.as_slice(),))?;
        file.call_method1(intern!(py, "seek"), (0,))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call1((&file, "gxhash64"))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash64()?
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}

#[test]
fn test_hashlib_file_digest_real_file_partial_seek() -> PyResult<()> {
    pytest!(py, {
        let file = py
            .import(intern!(py, "tempfile"))?
            .getattr(intern!(py, "NamedTemporaryFile"))?
            .call0()?;

        file.call_method1(intern!(py, "write"), (b"hello world",))?;
        file.call_method1(intern!(py, "seek"), (5,))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call1((&file, "gxhash64"))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = py
            .import_hashlib_gxhash64()?
            .call1((b" world",))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}

#[test]
fn test_hashlib_gxhash32_issubclass_hashlib_hash() -> PyResult<()> {
    pytest!(py, {
        let md5_type = py
            .import(intern!(py, "hashlib"))?
            .call_method0(intern!(py, "md5"))?
            .getattr(intern!(py, "__class__"))?;

        let gxhash_type = py
            .import_hashlib_gxhash32()?
            .call0()?
            .getattr(intern!(py, "__class__"))?;

        assert!(gxhash_type.cast::<PyType>()?.is_subclass(&md5_type)?);
    })
}

#[test]
fn test_hashlib_gxhash64_issubclass_hashlib_hash() -> PyResult<()> {
    pytest!(py, {
        let md5_type = py
            .import(intern!(py, "hashlib"))?
            .call_method0(intern!(py, "md5"))?
            .getattr(intern!(py, "__class__"))?;

        let gxhash_type = py
            .import_hashlib_gxhash64()?
            .call0()?
            .getattr(intern!(py, "__class__"))?;

        assert!(gxhash_type.cast::<PyType>()?.is_subclass(&md5_type)?);
    })
}

#[test]
fn test_hashlib_gxhash128_issubclass_hashlib_hash() -> PyResult<()> {
    pytest!(py, {
        let md5_type = py
            .import(intern!(py, "hashlib"))?
            .call_method0(intern!(py, "md5"))?
            .getattr(intern!(py, "__class__"))?;

        let gxhash_type = py
            .import_hashlib_gxhash128()?
            .call0()?
            .getattr(intern!(py, "__class__"))?;

        assert!(gxhash_type.cast::<PyType>()?.is_subclass(&md5_type)?);
    })
}

#[quickcheck]
fn test_hashlib_file_digest_bytesio_with_callable(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let gxhash32_callable = py.import_hashlib_gxhash32()?;

        let file = py
            .import(intern!(py, "io"))?
            .getattr(intern!(py, "BytesIO"))?
            .call1((bytes.as_slice(),))?;

        let file_digest = py
            .import_hashlib_file_digest()?
            .call1((&file, &gxhash32_callable))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        let direct_digest = gxhash32_callable
            .call1((bytes.as_slice(),))?
            .call_method0(intern!(py, "hexdigest"))?
            .extract::<String>()?;

        assert_eq!(file_digest, direct_digest);
    })
}
