use gl;
use std::ffi::CStr;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, shader_type: gl::types::GLenum) -> 
            Result<Shader, String> {
        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        };
        let compile_result = Self::get_compile_result(id)?;
        Ok( Shader { id: compile_result })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    fn get_compile_result(id: u32) -> Result<u32, String> {
        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success); }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len); }
            let error = super::create_string_buffer(len as usize);
            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            Err(error.to_string_lossy().into_owned())
        } else { Ok(id) }
    }

    pub fn id(&self) -> gl::types::GLuint { self.id }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}
