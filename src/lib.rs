mod core;
mod hashlib;

pub use core::GxHash32;
pub use core::GxHash64;
pub use core::GxHash128;

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
