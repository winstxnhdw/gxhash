use pyo3::Bound;
use pyo3::IntoPyObjectExt;
use pyo3::PyResult;
use pyo3::Python;
use pyo3::buffer::PyBuffer;
use pyo3::exceptions::PyTypeError;
use pyo3::pyclass;
use pyo3::pyfunction;
use pyo3::pymethods;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "aarch64")]
use std::arch::aarch64::*;

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn hexdigest32(hash: u32) -> String {
    let mut hex = Vec::<u8>::with_capacity(8);

    unsafe {
        let table = _mm_setr_epi8(
            b'0' as i8, b'1' as i8, b'2' as i8, b'3' as i8, b'4' as i8, b'5' as i8, b'6' as i8, b'7' as i8, b'8' as i8,
            b'9' as i8, b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8, b'e' as i8, b'f' as i8,
        );

        let input = _mm_cvtsi32_si128(hash as i32);
        let mask = _mm_set1_epi8(0x0F);
        let lo = _mm_and_si128(input, mask);
        let hi = _mm_and_si128(_mm_srli_epi16(input, 4), mask);
        _mm_storel_epi64(
            hex.as_mut_ptr().cast(),
            _mm_shuffle_epi8(table, _mm_unpacklo_epi8(hi, lo)),
        );

        hex.set_len(8);
        String::from_utf8_unchecked(hex)
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn hexdigest64(hash: u64) -> String {
    let mut hex = Vec::<u8>::with_capacity(16);

    unsafe {
        let table = _mm_setr_epi8(
            b'0' as i8, b'1' as i8, b'2' as i8, b'3' as i8, b'4' as i8, b'5' as i8, b'6' as i8, b'7' as i8, b'8' as i8,
            b'9' as i8, b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8, b'e' as i8, b'f' as i8,
        );

        let input = _mm_cvtsi64_si128(hash as i64);
        let mask = _mm_set1_epi8(0x0F);
        let lo = _mm_and_si128(input, mask);
        let hi = _mm_and_si128(_mm_srli_epi16(input, 4), mask);

        _mm_storeu_si128(
            hex.as_mut_ptr().cast(),
            _mm_shuffle_epi8(table, _mm_unpacklo_epi8(hi, lo)),
        );

        hex.set_len(16);
        String::from_utf8_unchecked(hex)
    }
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
fn hexdigest128(hash: u128) -> String {
    let mut hex = Vec::<u8>::with_capacity(32);

    unsafe {
        let table = _mm_setr_epi8(
            b'0' as i8, b'1' as i8, b'2' as i8, b'3' as i8, b'4' as i8, b'5' as i8, b'6' as i8, b'7' as i8, b'8' as i8,
            b'9' as i8, b'a' as i8, b'b' as i8, b'c' as i8, b'd' as i8, b'e' as i8, b'f' as i8,
        );

        let input = _mm_set_epi64x((hash >> 64) as i64, hash as i64);
        let mask = _mm_set1_epi8(0x0F);
        let lo = _mm_and_si128(input, mask);
        let hi = _mm_and_si128(_mm_srli_epi16(input, 4), mask);
        let buffer = hex.as_mut_ptr();
        _mm_storeu_si128(buffer.cast(), _mm_shuffle_epi8(table, _mm_unpacklo_epi8(hi, lo)));
        _mm_storeu_si128(
            buffer.add(16).cast(),
            _mm_shuffle_epi8(table, _mm_unpackhi_epi8(hi, lo)),
        );

        hex.set_len(32);
        String::from_utf8_unchecked(hex)
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn hexdigest32(hash: u32) -> String {
    let mut hex = Vec::<u8>::with_capacity(8);

    unsafe {
        let table = vld1q_u8(b"0123456789abcdef".as_ptr());
        let input = vcombine_u8(vcreate_u8(hash as u64), vcreate_u8(0));
        let hi = vshrq_n_u8(input, 4);
        let lo = vandq_u8(input, vdupq_n_u8(0x0F));
        vst1_u8(hex.as_mut_ptr(), vget_low_u8(vqtbl1q_u8(table, vzip1q_u8(hi, lo))));

        hex.set_len(8);
        String::from_utf8_unchecked(hex)
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn hexdigest64(hash: u64) -> String {
    let mut hex = Vec::<u8>::with_capacity(16);

    unsafe {
        let table = vld1q_u8(b"0123456789abcdef".as_ptr());
        let input = vcombine_u8(vcreate_u8(hash), vcreate_u8(0));
        let hi = vshrq_n_u8(input, 4);
        let lo = vandq_u8(input, vdupq_n_u8(0x0F));
        vst1q_u8(hex.as_mut_ptr(), vqtbl1q_u8(table, vzip1q_u8(hi, lo)));

        hex.set_len(16);
        String::from_utf8_unchecked(hex)
    }
}

#[cfg(target_arch = "aarch64")]
#[inline(always)]
fn hexdigest128(hash: u128) -> String {
    let mut hex = Vec::<u8>::with_capacity(32);

    unsafe {
        let table = vld1q_u8(b"0123456789abcdef".as_ptr());
        let input = vcombine_u8(vcreate_u8(hash as u64), vcreate_u8((hash >> 64) as u64));
        let hi = vshrq_n_u8(input, 4);
        let lo = vandq_u8(input, vdupq_n_u8(0x0F));
        let buffer = hex.as_mut_ptr();
        vst1q_u8(buffer, vqtbl1q_u8(table, vzip1q_u8(hi, lo)));
        vst1q_u8(buffer.add(16), vqtbl1q_u8(table, vzip2q_u8(hi, lo)));

        hex.set_len(32);
        String::from_utf8_unchecked(hex)
    }
}

trait PyBufferExt {
    fn as_bytes(&self, _: Python) -> &[u8];
}

impl PyBufferExt for PyBuffer<u8> {
    fn as_bytes(&self, _: Python) -> &[u8] {
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
        let error = PyTypeError::new_err(r#"Cannot instantiate Protocol class "HASH""#);
        Err(error)
    }
}

#[pymethods]
impl Buffer {
    #[new]
    fn new() -> PyResult<Self> {
        let error = PyTypeError::new_err(r#"Cannot instantiate Protocol class "Buffer""#);
        Err(error)
    }
}

macro_rules! impl_hashlib {
    ($name:ident, $function_name:ident, $digest_size:expr, $hasher:path, $hexdigest:path) => {
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

            fn digest(&self, py: Python) -> [u8; $digest_size] {
                $hasher(self.buffer.as_bytes(py), self.seed).to_le_bytes()
            }

            fn hexdigest(&self, py: Python) -> String {
                $hexdigest($hasher(self.buffer.as_bytes(py), self.seed))
            }

            fn update(&mut self, py: Python, data: PyBuffer<u8>) -> PyResult<()> {
                let slice = self.buffer.as_bytes(py);
                let new_slice = data.as_bytes(py);

                let mut combined = Vec::with_capacity(slice.len() + new_slice.len());
                combined.extend_from_slice(slice);
                combined.extend_from_slice(new_slice);
                self.buffer = PyBuffer::get(&combined.into_bound_py_any(py)?)?;

                Ok(())
            }

            fn copy(&self, py: Python) -> PyResult<Self> {
                let new_hashlib = Self {
                    seed: self.seed,
                    buffer: PyBuffer::get(&self.buffer.as_bytes(py).into_bound_py_any(py)?)?,
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
            let buffer = data.map_or_else(|| PyBuffer::get(&b"".into_bound_py_any(py)?), Ok)?;
            Ok($name { seed, buffer })
        }
    };
}

impl_hashlib!(GxHashLib32, gxhash32, 4, gxhash_core::gxhash32, hexdigest32);
impl_hashlib!(GxHashLib64, gxhash64, 8, gxhash_core::gxhash64, hexdigest64);
impl_hashlib!(GxHashLib128, gxhash128, 16, gxhash_core::gxhash128, hexdigest128);

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
#[pyo3::pymodule(submodule, name = "hashlib", gil_used = false)]
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
