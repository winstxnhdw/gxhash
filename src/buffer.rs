use pyo3::buffer::PyBuffer;

pub(crate) trait PyBufferExt {
    fn as_bytes(&self) -> &[u8];
}

impl PyBufferExt for PyBuffer<u8> {
    fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.buf_ptr() as *const u8, self.len_bytes()) }
    }
}
