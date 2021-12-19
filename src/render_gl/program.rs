use gl;
use std::ffi::{ CString, CStr };
use super::Shader;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe { gl::AttachShader(program_id, shader.id()); }
        }
        unsafe { gl::LinkProgram(program_id); }

        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success); }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len); }
            let error = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            Err(error.to_string_lossy().into_owned())
        } else {
            for shader in shaders {
                unsafe { gl::DetachShader(program_id, shader.id()); }
            }
            Ok(Program { id: program_id })
        }
    }

    pub fn id(&self) -> gl::types::GLuint { self.id }

    pub fn run(&self) { unsafe { gl::UseProgram(self.id); } }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
