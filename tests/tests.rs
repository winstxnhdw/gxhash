use gxhash::gxhash_py;
use pyo3::Bound;
use pyo3::PyAny;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyInt;
use pyo3::types::PyModule;
use pyo3::types::PyNone;
use quickcheck_macros::quickcheck;

static ONCE: std::sync::Once = std::sync::Once::new();

macro_rules! pytest {
    ($py:ident, $body:block) => {{
        ONCE.call_once(|| {
            pyo3::append_to_inittab!(gxhash_py);
            Python::initialize();
        });

        Python::attach(|$py| -> PyResult<()> {
            $body
            Ok(())
        })
    }};
}

trait PythonExt<'p> {
    fn import_asyncio(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash(&self) -> PyResult<Bound<'_, PyModule>>;
    fn import_gxhash32(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhash64(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_gxhash128(&self) -> PyResult<Bound<'_, PyAny>>;
    fn import_hasher(&self) -> PyResult<Bound<'_, PyAny>>;
}

impl<'p> PythonExt<'p> for Python<'p> {
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

    fn import_gxhash(&self) -> PyResult<Bound<'_, PyModule>> {
        self.import(intern!(*self, "gxhash"))
    }

    fn import_gxhash32(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "GxHash32"))
    }

    fn import_gxhash64(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "GxHash64"))
    }

    fn import_gxhash128(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "GxHash128"))
    }

    fn import_hasher(&self) -> PyResult<Bound<'_, PyAny>> {
        self.import_gxhash()?.getattr(intern!(*self, "Hasher"))
    }
}

fn call_hash<'p, T>(py: Python<'p>, obj: &Bound<'p, PyAny>, bytes: &[u8]) -> PyResult<T>
where
    for<'s> T: pyo3::FromPyObject<'s, 's, Error = pyo3::PyErr>,
{
    obj.call_method1(intern!(py, "hash"), (bytes,))?.extract()
}

fn call_hash_async<'p, T>(py: Python<'p>, obj: &Bound<'p, PyAny>, bytes: &[u8]) -> PyResult<T>
where
    for<'s> T: pyo3::FromPyObject<'s, 's, Error = pyo3::PyErr>,
{
    py.import_asyncio()?
        .getattr(intern!(py, "run"))?
        .call1((obj.call_method1(intern!(py, "hash_async"), (bytes,))?,))?
        .extract()
}

#[test]
fn test_import_gxhash() -> PyResult<()> {
    pytest!(py, { assert!(py.import_gxhash()?.is_instance_of::<PyModule>()) })
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
fn test_t_co() -> PyResult<()> {
    pytest!(py, {
        let t_co = py.import_gxhash()?.getattr("T_co")?.call0()?;
        assert!(t_co.is_instance_of::<PyInt>())
    })
}

#[test]
fn test_uint32() -> PyResult<()> {
    pytest!(py, {
        let uint32 = py.import_gxhash()?.getattr("Uint32")?.call0()?;
        assert!(uint32.is_instance_of::<PyInt>())
    })
}

#[test]
fn test_uint64() -> PyResult<()> {
    pytest!(py, {
        let uint64 = py.import_gxhash()?.getattr("Uint64")?.call0()?;
        assert!(uint64.is_instance_of::<PyInt>())
    })
}

#[test]
fn test_uint128() -> PyResult<()> {
    pytest!(py, {
        let uint128 = py.import_gxhash()?.getattr("Uint128")?.call0()?;
        assert!(uint128.is_instance_of::<PyInt>())
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
fn test_gxhash32_hash(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash32()?.call((), Some(&seed))?;

        let result1 = call_hash::<u32>(py, &hasher, &bytes)?;
        let result2 = call_hash::<u32>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash64()?.call((), Some(&seed))?;

        let result1 = call_hash::<u64>(py, &hasher, &bytes)?;
        let result2 = call_hash::<u64>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash128()?.call((), Some(&seed))?;

        let result1 = call_hash::<u128>(py, &hasher, &bytes)?;
        let result2 = call_hash::<u128>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_async(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash32()?.call((), Some(&seed))?;

        let result1 = call_hash_async::<u32>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u32>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_async(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash64()?.call((), Some(&seed))?;

        let result1 = call_hash_async::<u64>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u64>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_async(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash128()?.call((), Some(&seed))?;

        let result1 = call_hash_async::<u128>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u128>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_sync_async_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash32()?.call((), Some(&seed))?;

        let result1 = call_hash::<u32>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u32>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_sync_async_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash64()?.call((), Some(&seed))?;

        let result1 = call_hash::<u64>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u64>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_sync_async_equality(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let hasher = py.import_gxhash128()?.call((), Some(&seed))?;

        let result1 = call_hash::<u128>(py, &hasher, &bytes)?;
        let result2 = call_hash_async::<u128>(py, &hasher, &bytes)?;

        assert_eq!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_seed_change(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash32()?;

        let result1 = call_hash::<u32>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash::<u32>(py, &hasher_class.call1((seed + 1,))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_seed_change(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash64()?;

        let result1 = call_hash::<u64>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash::<u64>(py, &hasher_class.call1((seed + 1,))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_seed_change(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash128()?;

        let result1 = call_hash::<u128>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash::<u128>(py, &hasher_class.call1((seed + 1,))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash32_hash_async_seed_change(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash32()?;

        let result1 = call_hash_async::<u32>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash_async::<u32>(py, &hasher_class.call1((seed + 1,))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash64_hash_async_seed_change(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash64()?;

        let result1 = call_hash_async::<u64>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash_async::<u64>(py, &hasher_class.call1((seed + 1,))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[quickcheck]
fn test_gxhash128_hash_async_seed_change(bytes: Vec<u8>) -> PyResult<()> {
    pytest!(py, {
        let seed = 42;
        let hasher_class = py.import_gxhash128()?;

        let result1 = call_hash_async::<u128>(py, &hasher_class.call1((seed,))?, &bytes)?;
        let result2 = call_hash_async::<u128>(py, &hasher_class.call1((seed + 1,))?, &bytes)?;

        assert_ne!(result1, result2);
    })
}

#[test]
fn test_gxhash32_hash_determinism() -> PyResult<()> {
    pytest!(py, {
        let bytes = [1u8, 2, 3];
        let obj = py.import_gxhash32()?.call1((42,))?;
        let result = call_hash::<u32>(py, &obj, &bytes)?;

        assert_eq!(result, 2205376180u32);
    })
}

#[test]
fn test_gxhash64_hash_determinism() -> PyResult<()> {
    pytest!(py, {
        let bytes = [1u8, 2, 3];
        let obj = py.import_gxhash64()?.call1((42,))?;
        let result = call_hash::<u64>(py, &obj, &bytes)?;

        assert_eq!(result, 14923488923042930356u64);
    })
}

#[test]
fn test_gxhash128_hash_determinism() -> PyResult<()> {
    pytest!(py, {
        let bytes = [1u8, 2, 3];
        let obj = py.import_gxhash128()?.call1((42,))?;
        let result = call_hash::<u128>(py, &obj, &bytes)?;

        assert_eq!(result, 77345409872630947185460848780960292532u128);
    })
}

#[test]
fn test_gxhash32_hash_async_determinism() -> PyResult<()> {
    pytest!(py, {
        let bytes = [1u8, 2, 3];
        let obj = py.import_gxhash32()?.call1((42,))?;
        let result = call_hash_async::<u32>(py, &obj, &bytes)?;

        assert_eq!(result, 2205376180u32);
    })
}

#[test]
fn test_gxhash64_hash_async_determinism() -> PyResult<()> {
    pytest!(py, {
        let bytes = [1u8, 2, 3];
        let obj = py.import_gxhash64()?.call1((42,))?;
        let result = call_hash_async::<u64>(py, &obj, &bytes)?;

        assert_eq!(result, 14923488923042930356u64);
    })
}

#[test]
fn test_gxhash128_hash_async_determinism() -> PyResult<()> {
    pytest!(py, {
        let bytes = [1u8, 2, 3];
        let obj = py.import_gxhash128()?.call1((42,))?;
        let result = call_hash_async::<u128>(py, &obj, &bytes)?;

        assert_eq!(result, 77345409872630947185460848780960292532u128);
    })
}

#[test]
fn test_hasher_instantiation() -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let error = py.import_hasher()?.call((), Some(&seed)).unwrap_err();

        assert!(error.is_instance_of::<pyo3::exceptions::PyTypeError>(py));
    })
}

#[test]
fn test_hasher_getitem_gxhash32() -> PyResult<()> {
    pytest!(py, {
        let hasher_type = py
            .import_hasher()?
            .call_method1(intern!(py, "__class_getitem__"), (py.import_gxhash32()?,))?;

        assert!(hasher_type.is_instance_of::<PyNone>());
    })
}

#[test]
fn test_hasher_getitem_gxhash64() -> PyResult<()> {
    pytest!(py, {
        let hasher_type = py
            .import_hasher()?
            .call_method1(intern!(py, "__class_getitem__"), (py.import_gxhash64()?,))?;

        assert!(hasher_type.is_instance_of::<PyNone>());
    })
}

#[test]
fn test_hasher_getitem_gxhash128() -> PyResult<()> {
    pytest!(py, {
        let hasher_type = py
            .import_hasher()?
            .call_method1(intern!(py, "__class_getitem__"), (py.import_gxhash128()?,))?;
        assert!(hasher_type.is_instance_of::<PyNone>());
    })
}
