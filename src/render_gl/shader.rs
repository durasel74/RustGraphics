use std::ffi;
use gl;
use gl::types::{ GLenum, GLuint };
use super::ShaderError;

/// Представляет шейдер
#[derive(Clone, Eq, PartialEq)]
pub struct Shader {
    id: u32,
    shader_type: GLenum,
}
impl Shader {
    /// Создает шейдер из строки с кодом. 
    /// Требуется указать тип шейдера для правильной компиляции.
    pub fn from_source(source: &str, shader_type: GLenum) -> Result<Shader, ShaderError> {
        let csource = ffi::CString::new(source);
        if let Result::Err(err) = csource {
            return Err(ShaderError::ConvertCStringError(err));
        }
        let csource = csource.unwrap();

        let id = unsafe { gl::CreateShader(shader_type) };
        unsafe {
            gl::ShaderSource(id, 1, &csource.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        };

        let compile_result = Self::get_compile_result(id)?;
        Ok( Shader { id: compile_result, shader_type })
    }

    // Возвращает идентификатор шейдера или ошибку при неудачной компиляции.
    fn get_compile_result(id: u32) -> Result<u32, ShaderError> {
        let mut success = 1;
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
            Err(ShaderError::CompileError(error.to_string_lossy().into_owned()))
        } else { Ok(id) }
    }

    /// Возвращает идентификатор шейдера.
    pub fn id(&self) -> GLuint { self.id }

    /// Возвращает тип шейдера.
    pub fn shader_type(&self) -> GLenum { self.shader_type }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}
