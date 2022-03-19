use std::fs;
use std::ffi;
use gl;
use gl::types::{ GLuint };
use cgmath::{ Vector3, Matrix, Matrix3, Matrix4 };
use super::Shader;
use super::ShaderError;

/// Представляет косвенный объект программы шейдера OpenGL.
#[derive(Clone, Eq, PartialEq)]
pub struct ShaderProgram {
    id: GLuint,
    vert_id: GLuint,
    frag_id: GLuint,
}
impl ShaderProgram {
    /// Создает программу шейдера из файлов с кодом шейдеров.
    pub fn from_files(vert_path: &str, frag_path: &str) -> Result<Self, ShaderError> {
        let load_result = fs::read_to_string(vert_path);
        let vert_source = match load_result {
            Ok(source) => source,
            Err(err) => return Err(ShaderError::FileError(err)),
        };

        let load_result = fs::read_to_string(frag_path);
        let frag_source = match load_result {
            Ok(source) => source,
            Err(err) => return Err(ShaderError::FileError(err)),
        };

        let vert_shader = Shader::from_source(&vert_source, gl::VERTEX_SHADER)?;
        let frag_shader = Shader::from_source(&frag_source, gl::FRAGMENT_SHADER)?;
        Self::from_shaders(&vert_shader, &frag_shader)
    }

    /// Создает программу шейдера на основе двух шейдеров.
    pub fn from_shaders(vert_shader: &Shader, frag_shader: &Shader) -> 
            Result<Self, ShaderError> {
        Self::check_shader_types(vert_shader, frag_shader)?;

        let id = unsafe { gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vert_shader.id());
            gl::AttachShader(id, frag_shader.id());
            gl::LinkProgram(id);
            gl::DetachShader(id, vert_shader.id());
            gl::DetachShader(id, frag_shader.id());
        }
        let link_result = Self::get_link_result(id)?;
        Ok(ShaderProgram { id: link_result, vert_id: vert_shader.id(), 
            frag_id: frag_shader.id() })
    }

    // Возвращает идентификатор программы шейдера или ошибку при неудачном связывании.
    fn get_link_result(id: u32) -> Result<u32, ShaderError>{
        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); }
            let error = super::create_string_buffer(len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar);
            }
            Err(ShaderError::LinkError(error.to_string_lossy().into_owned()))
        } else { Ok(id) }
    }

    // Проверяет типы входных шейдеров.
    fn check_shader_types(vert_shader: &Shader, frag_shader: &Shader) -> Result<(), ShaderError> {
        if vert_shader.shader_type() != gl::VERTEX_SHADER {
            let error_message = "Vertex shader type is not set correctly";
            return Err(ShaderError::TypeError(String::from(error_message)));
        }
        if frag_shader.shader_type() != gl::FRAGMENT_SHADER {
            let error_message = "Fragment shader type is not set correctly";
            return Err(ShaderError::TypeError(String::from(error_message)));
        }
        Ok(())
    }

    /// Возвращает идентификатор программы шейдера.
    pub fn id(&self) -> GLuint { self.id }

    /// Возвращает идентификатор вершинного шейдера.
    pub fn vert_id(&self) -> GLuint { self.vert_id }

    /// Возвращает идентификатор фрагментного шейдера.
    pub fn frag_id(&self) -> GLuint { self.frag_id }

    /// Выполняет программу шейдера.
    pub fn use_(&self) { 
        unsafe { gl::UseProgram(self.id); } 
    }

    pub fn set_uniform_int(&self, field_name: &str, value: i32) {
        let cfield_name = ffi::CString::new(field_name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id(), cfield_name.as_ptr());
            gl::Uniform1i(location, value);
        }
    }

    pub fn set_uniform_float(&self, field_name: &str, value: f32) {
        let cfield_name = ffi::CString::new(field_name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id(), cfield_name.as_ptr());
            gl::Uniform1f(location, value);
        }
    }

    pub fn set_uniform_vector(&self, field_name: &str, value: &Vector3<f32>) {
        let cfield_name = ffi::CString::new(field_name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id(), cfield_name.as_ptr());
            gl::Uniform3f(location, value[0], value[1], value[2]);
        }
    }

    pub fn set_uniform_matrix3(&self, field_name: &str, value: &Matrix3<f32>) {
        let cfield_name = ffi::CString::new(field_name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id(), cfield_name.as_ptr());
            gl::UniformMatrix3fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }

    pub fn set_uniform_matrix4(&self, field_name: &str, value: &Matrix4<f32>) {
        let cfield_name = ffi::CString::new(field_name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.id(), cfield_name.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr());
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}
