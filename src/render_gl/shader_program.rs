use gl;
use gl::types::{ GLuint };

pub struct ShaderProgram {
    id: GLuint,
    vert_id: GLuint,
    frag_id: GLuint,
}

impl ShaderProgram {

    pub fn from_shaders(shaders: &[&Shader]) -> Result<Program, String> {

        
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

    

    pub fn id(&self) -> GLuint { self.id }
    pub fn vert_id(&self) -> GLuint { self.vert_id }
    pub fn frag_id(&self) -> GLuint { self.frag_id }


}