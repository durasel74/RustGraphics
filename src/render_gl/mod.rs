mod shader;
mod program;

pub use shader::Shader;
pub use program::Program;

use std::ffi::CString;

pub fn create_string_buffer(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
