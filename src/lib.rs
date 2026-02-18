mod core;
mod hashlib;

pub use core::GxHash32;
pub use core::GxHash64;
pub use core::GxHash128;
/// gxhash.core — core Python bindings for GxHash
///
/// This module contains the core Python bindings for GxHash, a blazingly fast and robust non-cryptographic hashing algorithm.
///
/// * GxHash32  - a class for computing 32-bit hashes
/// * GxHash64  - a class for computing 64-bit hashes
/// * GxHash128 - a class for computing 128-bit hashes
///
/// Each class provides methods for hashing byte sequences both synchronously and asynchronously.
///
/// * GxHash32(seed: int)
/// * hash(bytes: bytes) -> Uint32
/// * hash_async(bytes: bytes) -> Awaitable[Uint32]
///
/// * GxHash64(seed: int)
/// * hash(bytes: bytes) -> Uint64
/// * hash_async(bytes: bytes) -> Awaitable[Uint64]
///
/// * GxHash128(seed: int)
/// * hash(bytes: bytes) -> Uint128
/// * hash_async(bytes: bytes) -> Awaitable[Uint128]
///
#[pyo3::pymodule(name = "gxhash", gil_used = false)]
pub mod gxhash_py {
    use pyo3::types::PyAnyMethods;

    #[pymodule_export]
    use super::core::core_module;
    #[pymodule_export]
    use super::hashlib::hashlib_module;

    #[pymodule_init]
    fn init(m: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
        let py = m.py();
        let modules = py.import("sys")?.getattr("modules")?;

        modules.set_item("gxhash.core", m.getattr("core")?)?;
        modules.set_item("gxhash.gxhashlib", m.getattr("gxhashlib")?)
    }
}
