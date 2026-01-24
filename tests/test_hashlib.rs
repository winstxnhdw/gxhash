use crate::helpers::PythonExt;
use crate::pytest;
use gxhash::gxhash_py;
use pyo3::PyResult;
use pyo3::intern;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;
use quickcheck_macros::quickcheck;

#[test]
fn test_import_gxhash_hashlib() -> PyResult<()> {
    pytest!(py, {
        assert!(py.import_gxhash_hashlib()?.is_instance_of::<pyo3::types::PyModule>())
    })
}

#[test]
fn test_hashlib_hash_instantiation() -> PyResult<()> {
    pytest!(py, {
        let error = py
            .import_gxhash_hashlib()?
            .getattr(intern!(py, "HASH"))?
            .call0()
            .unwrap_err();

        assert!(error.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
    })
}

#[test]
fn test_hashlib_buffer_instantiation() -> PyResult<()> {
    pytest!(py, {
        let error = py
            .import_gxhash_hashlib()?
            .getattr(intern!(py, "Buffer"))?
            .call0()
            .unwrap_err();

        assert!(error.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
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
fn test_hashlib_gxhash32_hexdigest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash32()?.call1((bytes.as_slice(),))?;
        let hasher2 = py.import_hashlib_gxhash32()?.call1((bytes.as_slice(),))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest1, hexdigest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash64_hexdigest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash64()?.call1((bytes.as_slice(),))?;
        let hasher2 = py.import_hashlib_gxhash64()?.call1((bytes.as_slice(),))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest1, hexdigest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash128_hexdigest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash128()?.call1((bytes.as_slice(),))?;
        let hasher2 = py.import_hashlib_gxhash128()?.call1((bytes.as_slice(),))?;

        let hexdigest1 = hasher1.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let hexdigest2 = hasher2.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;

        assert_eq!(hexdigest1, hexdigest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash32_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash32()?.call1((bytes.as_slice(),))?;
        let hasher2 = py.import_hashlib_gxhash32()?.call1((bytes.as_slice(),))?;

        let digest1 = hasher1.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;
        let digest2 = hasher2.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;

        assert_eq!(digest1, digest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash64_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash64()?.call1((bytes.as_slice(),))?;
        let hasher2 = py.import_hashlib_gxhash64()?.call1((bytes.as_slice(),))?;

        let digest1 = hasher1.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;
        let digest2 = hasher2.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;

        assert_eq!(digest1, digest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash128_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher1 = py.import_hashlib_gxhash128()?.call1((bytes.as_slice(),))?;
        let hasher2 = py.import_hashlib_gxhash128()?.call1((bytes.as_slice(),))?;

        let digest1 = hasher1.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;
        let digest2 = hasher2.call_method0(intern!(py, "digest"))?.extract::<Vec<u8>>()?;

        assert_eq!(digest1, digest2);
    })
}

#[quickcheck]
fn test_hashlib_gxhash32_hexdigest_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash32()?.call1((bytes.as_slice(),))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let digest: String = hasher
            .call_method0(intern!(py, "digest"))?
            .extract::<Vec<u8>>()?
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();

        assert_eq!(hexdigest, digest);
    })
}

#[quickcheck]
fn test_hashlib_gxhash64_hexdigest_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash64()?.call1((bytes.as_slice(),))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let digest: String = hasher
            .call_method0(intern!(py, "digest"))?
            .extract::<Vec<u8>>()?
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();

        assert_eq!(hexdigest, digest);
    })
}

#[quickcheck]
fn test_hashlib_gxhash128_hexdigest_digest_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let hasher = py.import_hashlib_gxhash128()?.call1((bytes.as_slice(),))?;
        let hexdigest = hasher.call_method0(intern!(py, "hexdigest"))?.extract::<String>()?;
        let digest: String = hasher
            .call_method0(intern!(py, "digest"))?
            .extract::<Vec<u8>>()?
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
