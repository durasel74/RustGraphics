use gl;
use gl::types::{ GLuint };
use image::GenericImageView;
use super::ShaderError;

/// Представляет косвенный объект текстуры OpenGL.
#[derive(Clone, Eq, PartialEq)]
pub struct Texture {
    pub id: GLuint,
}
impl Texture {
    /// Создает текстуру из файла с изображением.
    pub fn from_file(file_name: &str) -> Result<Self, ShaderError> {
        let open_result = image::open(file_name);
        let img = match  open_result {
            Ok(img) => img.flipv(),
            Err(err) => return Err(ShaderError::ImageOpenError(err)),
        };
        let img_size = img.dimensions();

        let pixels_colors: Vec<_> = img.pixels().map(|pixel| { pixel.2 }).collect();
        let mut pixels_data = vec![];
        for i in pixels_colors.iter() {
            match i {
                image::Rgba(values) => { 
                    pixels_data.push(values[0]);
                    pixels_data.push(values[1]);
                    pixels_data.push(values[2]);
                    pixels_data.push(values[3]);
                }
            }
        }
        Self::from_bytes(&pixels_data, img_size)
    }

    /// Создает текстуру из байтов цвета изображения.
    pub fn from_bytes(pixels_data: &Vec<u8>, img_size: (u32, u32)) -> Result<Self, ShaderError> {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);

            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, img_size.0 as i32, 
                img_size.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, 
                pixels_data.as_ptr() as *const gl::types::GLvoid);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Ok(Texture { id: texture_id })
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}
