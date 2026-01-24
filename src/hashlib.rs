use pyo3::Bound;
use pyo3::IntoPyObjectExt;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::buffer::PyBuffer;
use pyo3::prelude::pyclass;
use pyo3::prelude::pymethods;
use pyo3::pyfunction;
use pyo3::types::PyBytes;

const HEX_TABLE: [[u8; 2]; 256] = [
    *b"00", *b"01", *b"02", *b"03", *b"04", *b"05", *b"06", *b"07", *b"08", *b"09", *b"0a", *b"0b", *b"0c", *b"0d",
    *b"0e", *b"0f", *b"10", *b"11", *b"12", *b"13", *b"14", *b"15", *b"16", *b"17", *b"18", *b"19", *b"1a", *b"1b",
    *b"1c", *b"1d", *b"1e", *b"1f", *b"20", *b"21", *b"22", *b"23", *b"24", *b"25", *b"26", *b"27", *b"28", *b"29",
    *b"2a", *b"2b", *b"2c", *b"2d", *b"2e", *b"2f", *b"30", *b"31", *b"32", *b"33", *b"34", *b"35", *b"36", *b"37",
    *b"38", *b"39", *b"3a", *b"3b", *b"3c", *b"3d", *b"3e", *b"3f", *b"40", *b"41", *b"42", *b"43", *b"44", *b"45",
    *b"46", *b"47", *b"48", *b"49", *b"4a", *b"4b", *b"4c", *b"4d", *b"4e", *b"4f", *b"50", *b"51", *b"52", *b"53",
    *b"54", *b"55", *b"56", *b"57", *b"58", *b"59", *b"5a", *b"5b", *b"5c", *b"5d", *b"5e", *b"5f", *b"60", *b"61",
    *b"62", *b"63", *b"64", *b"65", *b"66", *b"67", *b"68", *b"69", *b"6a", *b"6b", *b"6c", *b"6d", *b"6e", *b"6f",
    *b"70", *b"71", *b"72", *b"73", *b"74", *b"75", *b"76", *b"77", *b"78", *b"79", *b"7a", *b"7b", *b"7c", *b"7d",
    *b"7e", *b"7f", *b"80", *b"81", *b"82", *b"83", *b"84", *b"85", *b"86", *b"87", *b"88", *b"89", *b"8a", *b"8b",
    *b"8c", *b"8d", *b"8e", *b"8f", *b"90", *b"91", *b"92", *b"93", *b"94", *b"95", *b"96", *b"97", *b"98", *b"99",
    *b"9a", *b"9b", *b"9c", *b"9d", *b"9e", *b"9f", *b"a0", *b"a1", *b"a2", *b"a3", *b"a4", *b"a5", *b"a6", *b"a7",
    *b"a8", *b"a9", *b"aa", *b"ab", *b"ac", *b"ad", *b"ae", *b"af", *b"b0", *b"b1", *b"b2", *b"b3", *b"b4", *b"b5",
    *b"b6", *b"b7", *b"b8", *b"b9", *b"ba", *b"bb", *b"bc", *b"bd", *b"be", *b"bf", *b"c0", *b"c1", *b"c2", *b"c3",
    *b"c4", *b"c5", *b"c6", *b"c7", *b"c8", *b"c9", *b"ca", *b"cb", *b"cc", *b"cd", *b"ce", *b"cf", *b"d0", *b"d1",
    *b"d2", *b"d3", *b"d4", *b"d5", *b"d6", *b"d7", *b"d8", *b"d9", *b"da", *b"db", *b"dc", *b"dd", *b"de", *b"df",
    *b"e0", *b"e1", *b"e2", *b"e3", *b"e4", *b"e5", *b"e6", *b"e7", *b"e8", *b"e9", *b"ea", *b"eb", *b"ec", *b"ed",
    *b"ee", *b"ef", *b"f0", *b"f1", *b"f2", *b"f3", *b"f4", *b"f5", *b"f6", *b"f7", *b"f8", *b"f9", *b"fa", *b"fb",
    *b"fc", *b"fd", *b"fe", *b"ff",
];

trait PyBufferExt {
    unsafe fn as_bytes(&self) -> &[u8];
}

impl PyBufferExt for PyBuffer<u8> {
    unsafe fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buf_ptr() as *const u8, self.len_bytes()) }
    }
}

#[cfg_attr(
    not(any(Py_3_8, Py_3_9)),
    pyclass(name = "HASH", module = "_hashlib", frozen, immutable_type, subclass)
)]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(name = "HASH", module = "_hashlib", frozen, subclass))]
struct Hash;

#[cfg_attr(not(any(Py_3_8, Py_3_9)), pyclass(frozen, immutable_type, subclass))]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(module = "gxhash.hashlib", frozen, subclass))]
struct Buffer;

#[cfg_attr(
    not(any(Py_3_8, Py_3_9)),
    pyclass(name = "HASH", module = "_hashlib", immutable_type)
)]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(name = "HASH", module = "_hashlib"))]
pub(crate) struct GxHashLib32 {
    seed: i64,
    buffer: PyBuffer<u8>,
}

#[cfg_attr(
    not(any(Py_3_8, Py_3_9)),
    pyclass(name = "HASH", module = "_hashlib", immutable_type)
)]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(name = "HASH", module = "_hashlib"))]
pub(crate) struct GxHashLib64 {
    seed: i64,
    buffer: PyBuffer<u8>,
}

#[cfg_attr(
    not(any(Py_3_8, Py_3_9)),
    pyclass(name = "HASH", module = "_hashlib", immutable_type)
)]
#[cfg_attr(any(Py_3_8, Py_3_9), pyclass(name = "HASH", module = "_hashlib"))]
pub(crate) struct GxHashLib128 {
    seed: i64,
    buffer: PyBuffer<u8>,
}

#[pymethods]
impl Hash {
    #[new]
    fn new() -> PyResult<Self> {
        let error = pyo3::exceptions::PyTypeError::new_err(r#"Cannot instantiate Protocol class "HASH""#);
        Err(error)
    }
}

#[pymethods]
impl Buffer {
    #[new]
    fn new() -> PyResult<Self> {
        let error = pyo3::exceptions::PyTypeError::new_err(r#"Cannot instantiate Protocol class "Buffer""#);
        Err(error)
    }
}

macro_rules! impl_hashlib {
    ($name:ident, $function_name:ident, $digest_size:expr, $hasher:path) => {
        #[pymethods]
        impl $name {
            #[getter]
            fn name(&self) -> &'static str {
                stringify!($function_name)
            }

            #[getter]
            fn digest_size(&self) -> usize {
                $digest_size
            }

            #[getter]
            fn block_size(&self) -> usize {
                1
            }

            fn digest<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
                let bytes = py.detach(|| {
                    let slice = unsafe { self.buffer.as_bytes() };
                    $hasher(slice, self.seed).to_le_bytes()
                });

                Ok(PyBytes::new(py, &bytes))
            }

            fn hexdigest(&self, py: Python) -> PyResult<String> {
                py.detach(|| {
                    let slice = unsafe { self.buffer.as_bytes() };
                    let bytes = $hasher(slice, self.seed).to_le_bytes();
                    let mut hex = String::with_capacity(bytes.len() * 2);

                    for byte in bytes {
                        let pair = HEX_TABLE[byte as usize];
                        hex.push(pair[0] as char);
                        hex.push(pair[1] as char);
                    }

                    Ok(hex)
                })
            }

            fn update(&mut self, py: Python, data: PyBuffer<u8>) -> PyResult<()> {
                let slice = unsafe { self.buffer.as_bytes() };
                let new_slice = unsafe { data.as_bytes() };

                let mut combined = Vec::with_capacity(slice.len() + new_slice.len());
                combined.extend_from_slice(slice);
                combined.extend_from_slice(new_slice);
                self.buffer = PyBuffer::get(&combined.into_bound_py_any(py)?)?;

                Ok(())
            }

            fn copy(&self, py: Python) -> PyResult<Self> {
                let slice = unsafe { self.buffer.as_bytes() };
                let new_hashlib = Self {
                    seed: self.seed,
                    buffer: PyBuffer::get(&PyBytes::new(py, slice))?,
                };

                Ok(new_hashlib)
            }
        }

        #[pyfunction]
        #[pyo3(signature = (data = None, *, seed = 0, usedforsecurity = false, **_kwargs))]
        pub(crate) fn $function_name(
            py: Python<'_>,
            data: Option<PyBuffer<u8>>,
            seed: i64,
            usedforsecurity: bool,
            _kwargs: Option<Bound<'_, pyo3::types::PyDict>>,
        ) -> PyResult<$name> {
            let _ = usedforsecurity;
            let buffer = match data {
                Some(buf) => buf,
                None => PyBuffer::get(&PyBytes::new(py, b""))?,
            };

            Ok($name { seed, buffer })
        }
    };
}

impl_hashlib!(GxHashLib32, gxhash32, 4, gxhash_core::gxhash32);
impl_hashlib!(GxHashLib64, gxhash64, 8, gxhash_core::gxhash64);
impl_hashlib!(GxHashLib128, gxhash128, 16, gxhash_core::gxhash128);

/// gxhash.hashlib — hashlib-compatible GxHash API
///
/// This module contains the hashlib-compatible API for GxHash.
///
/// * gxhash32  - a hashlib-compatible class for computing 32-bit hashes
/// * gxhash64  - a hashlib-compatible class for computing 64-bit hashes
/// * gxhash128 - a hashlib-compatible class for computing 128-bit hashes
///
/// The functions provide a compatible interface with Python's built-in hashlib module.
///
/// * gxhash32(data: bytes = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH
/// * gxhash64(data: bytes = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH
/// * gxhash128(data: bytes = b"", *, seed: int = 0, usedforsecurity: bool = False, **kwargs: object) -> HASH
///
/// The HASH objects returned by these functions again provide the standard HASH methods and properties.
///
/// * name -> str
/// * digest_size -> int
/// * block_size -> int
/// * digest() -> bytes
/// * hexdigest() -> str
/// * update(data: bytes) -> None
/// * copy() -> HASH
///
#[pyo3::prelude::pymodule(submodule, name = "hashlib", gil_used = false)]
pub mod hashlib_module {
    #[pymodule_export]
    use super::Buffer;
    #[pymodule_export]
    use super::Hash;
    #[pymodule_export]
    use super::gxhash32;
    #[pymodule_export]
    use super::gxhash64;
    #[pymodule_export]
    use super::gxhash128;
}
