use std::ffi;
use std::fmt;
use std::error;

pub fn create_string_buffer(len: usize) -> ffi::CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { ffi::CString::from_vec_unchecked(buffer) }
}

#[derive(Debug)]
pub enum ShaderError {
    CompileError(String),
    LinkError(String),
    ConvertCStringError(ffi::NulError),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::CompileError(ref err) => write!(f, "Compile error: {}", err),
            ShaderError::LinkError(ref err) => write!(f, "Link error: {}", err),
            ShaderError::ConvertCStringError(ref err) => write!(f, "Convert CString error: {}", err),
        }
    }
}

impl error::Error for ShaderError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ShaderError::CompileError(_) => None,
            ShaderError::LinkError(_) => None,
            ShaderError::ConvertCStringError(ref err) => Some(err),
        }
    }
}
