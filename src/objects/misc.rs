use std::io;
use std::ffi;
use std::fmt;
use std::error;
use image;

pub fn create_string_buffer(len: usize) -> ffi::CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { ffi::CString::from_vec_unchecked(buffer) }
}

#[derive(Debug)]
pub enum ShaderError {
    CompileError(String),
    TypeError(String),
    LinkError(String),
    FrameBufferError(String),
    FileError(io::Error),
    ConvertCStringError(ffi::NulError),
    ImageOpenError(image::ImageError),
}

impl fmt::Display for ShaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ShaderError::CompileError(ref err) => write!(f, "Compile error: {}", err),
            ShaderError::TypeError(ref err) => write!(f, "Shader type error: {}", err),
            ShaderError::LinkError(ref err) => write!(f, "Link error: {}", err),
            ShaderError::FrameBufferError(ref err) => write!(f, "Frame buffer error: {}", err),
            ShaderError::FileError(ref err) => write!(f, "File error: {}", err),
            ShaderError::ConvertCStringError(ref err) => write!(f, "Convert CString error: {}", err),
            ShaderError::ImageOpenError(ref err) => write!(f, "Image open error: {}", err),
        }
    }
}

impl error::Error for ShaderError {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            ShaderError::CompileError(_) => None,
            ShaderError::TypeError(_) => None,
            ShaderError::LinkError(_) => None,
            ShaderError::FrameBufferError(_) => None,
            ShaderError::FileError(ref err) => Some(err),
            ShaderError::ConvertCStringError(ref err) => Some(err),
            ShaderError::ImageOpenError(ref err) => Some(err),
        }
    }
}
