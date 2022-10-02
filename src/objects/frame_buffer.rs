use gl;
use gl::types::{ GLuint };
use super::{ ShaderError, Texture }; //RenderBuffer

/// Представляет косвенный объект фрейм буфера OpenGL.
#[derive(Clone, Eq, PartialEq)]
pub struct FrameBuffer {
    pub fbo: GLuint,
    pub color_buffer: Texture,
    pub depth_stencil_buffer: Texture,

    // pub render_buffer: RenderBuffer
}
impl FrameBuffer {
    /// Создает новый фреймбуфер
    pub fn new(width: u32, height: u32) -> Result<Self, ShaderError> {
        let mut fbo = 0;
        let mut color_buffer: Texture;
        let mut depth_stencil_buffer: Texture;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            color_buffer = Texture::new_rgb(width, height, gl::RGB as i32);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, 
                gl::TEXTURE_2D, color_buffer.id, 0);

            depth_stencil_buffer = Texture::new_depth_stencil(width, height);
            gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, 
                gl::TEXTURE_2D, depth_stencil_buffer.id, 0);

            // gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, 
            //     gl::RENDERBUFFER, rbo);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err(ShaderError::FrameBufferError("Ошибка при создании фреймбуфера!".to_string()))
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        Ok(FrameBuffer { fbo, color_buffer, depth_stencil_buffer } )
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.fbo); }
    }
}
