use gxhash::gxhash_py;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::IntoPyDict;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyInt;
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

macro_rules! gxhash_import_test {
    ($name:ident, $attribute:expr) => {
        #[test]
        fn $name() -> PyResult<()> {
            pytest!(py, {
                let obj = py
                    .import(intern!(py, "gxhash"))?
                    .getattr(intern!(py, $attribute))?;

                assert!(obj.is_instance_of::<pyo3::types::PyAny>());
            })
        }
    };
}

macro_rules! gxhash_callable_test {
    ($name:ident, $attribute:expr, $ty:ty) => {
        #[test]
        fn $name() -> PyResult<()> {
            pytest!(py, {
                let obj = py
                    .import(intern!(py, "gxhash"))?
                    .getattr(intern!(py, $attribute))?
                    .call0()?;

                assert!(obj.is_instance_of::<$ty>());
            })
        }
    };
}

macro_rules! gxhash_getitem_test {
    ($name:ident, $hasher:expr) => {
        #[test]
        fn $name() -> PyResult<()> {
            pytest!(py, {
                let gxhash = py.import(intern!(py, "gxhash"))?;
                let hasher = gxhash.getattr(intern!(py, $hasher))?;

                gxhash
                    .getattr(intern!(py, "Hasher"))?
                    .call_method1(intern!(py, "__class_getitem__"), (hasher,))?;
            })
        }
    };
}

macro_rules! gxhash_hash_test {
    ($test_name:ident, $hasher:expr, $output_type:ty) => {
        #[quickcheck]
        fn $test_name(bytes: Vec<u8>) -> PyResult<()> {
            pytest!(py, {
                let seed = [("seed", 42)].into_py_dict(py)?;
                let hasher = py
                    .import(intern!(py, "gxhash"))?
                    .getattr(intern!(py, $hasher))?
                    .call((), Some(&seed))?;

                let result1 = hasher
                    .call_method1(intern!(py, "hash"), (&bytes,))?
                    .extract::<$output_type>()?;
                let result2 = hasher
                    .call_method1(intern!(py, "hash"), (&bytes,))?
                    .extract::<$output_type>()?;

                assert_eq!(result1, result2);
            })
        }
    };
}

macro_rules! gxhash_hash_async_test {
    ($test_name:ident, $hasher:expr, $output_type:ty) => {
        #[quickcheck]
        fn $test_name(bytes: Vec<u8>) -> PyResult<()> {
            pytest!(py, {
                let asyncio = py.import(intern!(py, "asyncio"))?;

                #[cfg(windows)]
                {
                    let policy = asyncio.getattr(intern!(py, "WindowsSelectorEventLoopPolicy"))?;
                    asyncio.call_method1(intern!(py, "set_event_loop_policy"), (policy.call0()?,))?;
                }

                let async_run = asyncio.getattr(intern!(py, "run"))?;
                let seed = [("seed", 42)].into_py_dict(py)?;
                let hasher = py
                    .import(intern!(py, "gxhash"))?
                    .getattr(intern!(py, $hasher))?
                    .call((), Some(&seed))?;

                let coroutine1 = hasher.call_method1(intern!(py, "hash_async"), (&bytes,))?;
                let coroutine2 = hasher.call_method1(intern!(py, "hash_async"), (&bytes,))?;

                assert_eq!(
                    async_run.call1((coroutine1,))?.extract::<$output_type>()?,
                    async_run.call1((coroutine2,))?.extract::<$output_type>()?,
                );
            })
        }
    };
}

macro_rules! gxhash_hash_determinism_test {
    ($test_name:ident, $hasher:expr, $output_type:ty, $expected:expr) => {
        #[test]
        fn $test_name() -> PyResult<()> {
            pytest!(py, {
                let bytes = [1u8, 2, 3];
                let result = py
                    .import(intern!(py, "gxhash"))?
                    .getattr($hasher)?
                    .call1((42,))?
                    .call_method1(intern!(py, "hash"), (&bytes,))?
                    .extract::<$output_type>()?;

                assert_eq!(result, $expected);
            })
        }
    };
}

macro_rules! gxhash_hash_async_determinism_test {
    ($test_name:ident, $hasher:expr, $output_type:ty, $expected:expr) => {
        #[test]
        fn $test_name() -> PyResult<()> {
            pytest!(py, {
                let bytes = [1u8, 2, 3];
                let coroutine = py
                    .import(intern!(py, "gxhash"))?
                    .getattr($hasher)?
                    .call1((42,))?
                    .call_method1(intern!(py, "hash_async"), (&bytes,))?;

                let result = py
                    .import(intern!(py, "asyncio"))?
                    .getattr(intern!(py, "run"))?
                    .call1((coroutine,))?
                    .extract::<$output_type>()?;

                assert_eq!(result, $expected);
            })
        }
    };
}

macro_rules! gxhash_hash_seed_test {
    ($test_name:ident, $hasher:expr, $output_type:ty) => {
        #[quickcheck]
        fn $test_name(bytes: Vec<u8>) -> PyResult<()> {
            pytest!(py, {
                let seed = 42;
                let hasher_class = py.import(intern!(py, "gxhash"))?.getattr(intern!(py, $hasher))?;

                let result1 = hasher_class
                    .call1((seed,))?
                    .call_method1(intern!(py, "hash"), (&bytes,))?
                    .extract::<$output_type>()?;
                let result2 = hasher_class
                    .call1((seed + 1,))?
                    .call_method1(intern!(py, "hash"), (&bytes,))?
                    .extract::<$output_type>()?;

                assert_ne!(result1, result2);
            })
        }
    };
}

macro_rules! gxhash_hash_async_seed_test {
    ($test_name:ident, $hasher:expr, $output_type:ty) => {
        #[quickcheck]
        fn $test_name(bytes: Vec<u8>) -> PyResult<()> {
            pytest!(py, {
                let asyncio = py.import(intern!(py, "asyncio"))?;

                #[cfg(windows)]
                {
                    let policy = asyncio.getattr(intern!(py, "WindowsSelectorEventLoopPolicy"))?;
                    asyncio.call_method1(intern!(py, "set_event_loop_policy"), (policy.call0()?,))?;
                }

                let seed = 42;
                let async_run = asyncio.getattr(intern!(py, "run"))?;
                let hasher_class = py.import(intern!(py, "gxhash"))?.getattr($hasher)?;

                let coroutine1 = hasher_class
                    .call1((seed,))?
                    .call_method1(intern!(py, "hash_async"), (&bytes,))?;
                let coroutine2 = hasher_class
                    .call1((seed + 1,))?
                    .call_method1(intern!(py, "hash_async"), (&bytes,))?;

                assert_ne!(
                    async_run.call1((coroutine1,))?.extract::<$output_type>()?,
                    async_run.call1((coroutine2,))?.extract::<$output_type>()?
                );
            })
        }
    };
}

#[test]
fn test_import_gxhash() -> PyResult<()> {
    pytest!(py, {
        let gxhash = py.import(intern!(py, "gxhash"))?;
        assert!(gxhash.is_instance_of::<pyo3::types::PyModule>());
    })
}

gxhash_import_test!(test_gxhash32_from_gxhash, "GxHash32");
gxhash_import_test!(test_gxhash64_from_gxhash, "GxHash64");
gxhash_import_test!(test_gxhash128_from_gxhash, "GxHash128");
gxhash_import_test!(test_hasher_from_gxhash, "Hasher");

gxhash_callable_test!(
    test_gxhash_async_error,
    "GxHashAsyncError",
    pyo3::exceptions::PyException
);
gxhash_callable_test!(test_t_co, "T_co", PyInt);
gxhash_callable_test!(test_uint32, "Uint32", PyInt);
gxhash_callable_test!(test_uint64, "Uint64", PyInt);
gxhash_callable_test!(test_uint128, "Uint128", PyInt);

gxhash_hash_test!(test_gxhash32_hash, "GxHash32", u32);
gxhash_hash_test!(test_gxhash64_hash, "GxHash64", u64);
gxhash_hash_test!(test_gxhash128_hash, "GxHash128", u128);

gxhash_hash_async_test!(test_gxhash32_hash_async, "GxHash32", u32);
gxhash_hash_async_test!(test_gxhash64_hash_async, "GxHash64", u64);
gxhash_hash_async_test!(test_gxhash128_hash_async, "GxHash128", u128);

gxhash_hash_determinism_test!(test_gxhash32_hash_determinism, "GxHash32", u32, 2205376180u32);
gxhash_hash_determinism_test!(test_gxhash64_hash_determinism, "GxHash64", u64, 14923488923042930356u64);
gxhash_hash_determinism_test!(
    test_gxhash128_hash_determinism,
    "GxHash128",
    u128,
    77345409872630947185460848780960292532u128
);

gxhash_hash_async_determinism_test!(test_gxhash32_hash_async_determinism, "GxHash32", u32, 2205376180u32);
gxhash_hash_async_determinism_test!(
    test_gxhash64_hash_async_determinism,
    "GxHash64",
    u64,
    14923488923042930356u64
);
gxhash_hash_async_determinism_test!(
    test_gxhash128_hash_async_determinism,
    "GxHash128",
    u128,
    77345409872630947185460848780960292532u128
);

gxhash_hash_seed_test!(test_gxhash32_seed, "GxHash32", u32);
gxhash_hash_seed_test!(test_gxhash64_seed, "GxHash64", u64);
gxhash_hash_seed_test!(test_gxhash128_seed, "GxHash128", u128);

gxhash_hash_async_seed_test!(test_gxhash32_seed_async, "GxHash32", u32);
gxhash_hash_async_seed_test!(test_gxhash64_seed_async, "GxHash64", u64);
gxhash_hash_async_seed_test!(test_gxhash128_seed_async, "GxHash128", u128);

#[test]
fn test_hasher_instantiation() -> PyResult<()> {
    pytest!(py, {
        let seed = [("seed", 42)].into_py_dict(py)?;
        let error = py
            .import(intern!(py, "gxhash"))?
            .getattr(intern!(py, "Hasher"))?
            .call((), Some(&seed))
            .unwrap_err();

        assert!(error.is_instance_of::<pyo3::exceptions::PyException>(py));
    })
}

gxhash_getitem_test!(test_hasher_getitem_gxhash32, "GxHash32");
gxhash_getitem_test!(test_hasher_getitem_gxhash64, "GxHash64");
gxhash_getitem_test!(test_hasher_getitem_gxhash128, "GxHash128");
