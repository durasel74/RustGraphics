use gl;
use super::Shader;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let id = unsafe { gl::CreateProgram() };
        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()); }
        }
        unsafe { gl::LinkProgram(id); }

        let link_result = Self::get_link_result(id)?;
        for shader in shaders {
            unsafe { gl::DetachShader(id, shader.id()); }
        }
        Ok(Program { id: link_result })
    }

    fn get_link_result(id: u32) -> Result<u32, String>{
        let mut success: gl::types::GLint = 1;
        unsafe { gl::GetProgramiv(id, gl::LINK_STATUS, &mut success); }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len); }
            let error = super::create_string_buffer(len as usize);
            unsafe {
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            Err(error.to_string_lossy().into_owned())
        } else { Ok(id) }
    }

    pub fn id(&self) -> gl::types::GLuint { self.id }

    pub fn run(&self) { 
        unsafe { gl::UseProgram(self.id); } 
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}
