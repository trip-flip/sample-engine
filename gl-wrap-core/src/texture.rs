use gl::{
    self,
    types::{
        GLuint
    }
};
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use std::path::Path;

/// A texture that gets wrapped onto a mesh.
#[derive(Debug)]
pub struct Texture {
    /// Id of the texture.
    id: GLuint,
    /// Width of the texture.
    width: GLuint,
    /// Height of the texture.
    height: GLuint
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); }
    }
}

impl Texture {
    /// Creates a new texture from a path. Only supports RGB formats.
    /// NOTE: May fail with images with alpha data (e.g. PNGs)
    // TODO: Introduce support for alpha data.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let image = Surface::from_file(path)?;
        let mut texture = Texture {
            id: 0,
            width: image.width(),
            height: image.height()
        };
        
        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            texture.linear_mipmap_nearest();
            gl::TexImage2D(gl::TEXTURE_2D, 
                           0, 
                           gl::RGB as i32, 
                           image.width() as i32, 
                           image.height() as i32, 
                           0, 
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           (*image.raw()).pixels);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(texture)
    }

    /// Enable the texture for drawing.
    pub fn enable(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    /// Set texture to linear filtering.
    pub fn linear(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }

    /// Set texture to nearest filtering.
    pub fn nearest(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
    }

    /// Set texture to linear filtering using nearest mipmap.
    pub fn linear_mipmap_nearest(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_NEAREST as i32);
        }
    }

    /// Set texture to nearest filtering using nearest mipmap.
    pub fn nearest_mipmap_nearest(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST_MIPMAP_NEAREST as i32);
        }
    }

    /// Set texture to linear filtering using linear combination of nearest mipmaps.
    pub fn linear_mipmap_linear(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        }
    }

    /// Set texture to nearest filtering using linear combination of nearest mipmaps.
    pub fn nearest_mipmap_linear(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST_MIPMAP_LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST_MIPMAP_LINEAR as i32);
        }
    }

    /// Get the width of the Texture.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the Texture.
    pub fn height(&self) -> u32 {
        self.height
    }
}
