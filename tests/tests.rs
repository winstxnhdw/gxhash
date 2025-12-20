use gxhash::gxhash_py;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::intern;
use pyo3::types::PyAnyMethods;
use pyo3::types::PyInt;
use serial_test::serial;

static ONCE: std::sync::Once = std::sync::Once::new();
static BYTES: [u8; 3] = [1u8, 2, 3];

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

macro_rules! gxhash_hash_test {
    ($test_name:ident, $hasher:expr, $output_type:ty, $expected:expr) => {
        #[test]
        fn $test_name() -> PyResult<()> {
            pytest!(py, {
                let result = py
                    .import(intern!(py, "gxhash"))?
                    .getattr($hasher)?
                    .call1((42,))?
                    .call_method1(intern!(py, "hash"), (BYTES,))?
                    .extract::<$output_type>()?;

                assert_eq!(result, $expected);
            })
        }
    };
}

macro_rules! gxhash_hash_async_test {
    ($test_name:ident, $hasher:expr, $output_type:ty, $expected:expr) => {
        #[serial]
        #[test]
        fn $test_name() -> PyResult<()> {
            pytest!(py, {
                let coroutine = py
                    .import(intern!(py, "gxhash"))?
                    .getattr($hasher)?
                    .call1((42,))?
                    .call_method1(intern!(py, "hash_async"), (BYTES,))?;

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
        #[test]
        fn $test_name() -> PyResult<()> {
            pytest!(py, {
                let seed = 42;
                let hasher_class = py.import(intern!(py, "gxhash"))?.getattr($hasher)?;

                let result1 = hasher_class
                    .call1((seed,))?
                    .call_method1(intern!(py, "hash"), (BYTES,))?
                    .extract::<$output_type>()?;

                let result2 = hasher_class
                    .call1((seed + 1,))?
                    .call_method1(intern!(py, "hash"), (BYTES,))?
                    .extract::<$output_type>()?;

                assert_ne!(result1, result2);
            })
        }
    };
}

macro_rules! gxhash_hash_async_seed_test {
    ($test_name:ident, $hasher:expr, $output_type:ty) => {
        #[serial]
        #[test]
        fn $test_name() -> PyResult<()> {
            pytest!(py, {
                let async_run = py.import(intern!(py, "asyncio"))?.getattr(intern!(py, "run"))?;
                let hasher_class = py.import(intern!(py, "gxhash"))?.getattr($hasher)?;
                let seed = 42;

                let coroutine1 = hasher_class
                    .call1((seed,))?
                    .call_method1(intern!(py, "hash_async"), (BYTES,))?;

                let coroutine2 = hasher_class
                    .call1((seed + 1,))?
                    .call_method1(intern!(py, "hash_async"), (BYTES,))?;

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

gxhash_hash_test!(test_gxhash32_hash, "GxHash32", u32, 2205376180u32);
gxhash_hash_test!(test_gxhash64_hash, "GxHash64", u64, 14923488923042930356u64);
gxhash_hash_test!(
    test_gxhash128_hash,
    "GxHash128",
    u128,
    77345409872630947185460848780960292532u128
);

gxhash_hash_async_test!(test_gxhash32_hash_async, "GxHash32", u32, 2205376180u32);
gxhash_hash_async_test!(test_gxhash64_hash_async, "GxHash64", u64, 14923488923042930356u64);
gxhash_hash_async_test!(
    test_gxhash128_hash_async,
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
