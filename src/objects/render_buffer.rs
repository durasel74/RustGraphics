// use gl;
// use gl::types::{ GLuint };
// use super::{ ShaderError, Texture };

// /// Представляет косвенный объект фрейм буфера OpenGL.
// #[derive(Clone, Eq, PartialEq)]
// pub struct RenderBuffer {
//     pub rbo: GLuint,

//     // pub color_buffer: Texture,
//     // pub depth_stencil_buffer: Texture,
// }
// impl RenderBuffer {
//     /// Создает новый фрейм буфер
//     pub fn new() -> Result<Self, ShaderError> {
//         let mut rbo = 0;

//         // let mut color_buffer: Texture;
//         // let mut depth_stencil_buffer: Texture;



//         unsafe {
//             gl::GenRenderbuffers(1, &mut rbo);
//             gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

//             gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH24_STENCIL8, 800, 600);

//             // if(glCheckFramebufferStatus(GL_FRAMEBUFFER) == GL_FRAMEBUFFER_COMPLETE)
//             //     // execute victory dance

//             color_buffer = Texture::new_rgb(800, 600, gl::RGBA as i32);
//             gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, 
//                 gl::TEXTURE_2D, color_buffer.id, 0);

//             depth_stencil_buffer = Texture::new_depth_stencil(800, 600);
//             gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_STENCIL_ATTACHMENT, 
//                 gl::TEXTURE_2D, depth_stencil_buffer.id, 0);
//         }
//         Ok(RenderBuffer { fbo: (), color_buffer: (), depth_stencil_buffer: () } { fbo, color_buffer, depth_stencil_buffer } )
//     }
// }

// impl Drop for RenderBuffer {
//     fn drop(&mut self) {
//         unsafe { 
//             gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
//             gl::DeleteFramebuffers(1, &self.fbo); 
//         }
//     }
// }
