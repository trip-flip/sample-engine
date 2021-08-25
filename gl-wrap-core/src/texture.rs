use std::path::Path;
use std::ffi::c_void;
use gl::{
    self,
    types::{
        GLuint
    }
};
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use sdl2::pixels::PixelFormatEnum as PFE;
use image::{self, DynamicImage as DI};

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
    /*pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let image = Surface::from_file(path)?;
        let format = match image.pixel_format_enum() {
            PFE::RGB332 |
            PFE::RGB444 |
            PFE::RGB555 => gl::RGB,

            PFE::RGBA4444 => gl::RGBA,

            _ => gl::RGB
        };
        

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
            /*gl::TexImage2D(gl::TEXTURE_2D, 
                           0, 
                           gl::RGB as i32, 
                           image.width() as i32, 
                           image.height() as i32, 
                           0, 
                           gl::RGB,
                           gl::UNSIGNED_BYTE,
                           (*image.raw()).pixels);*/
            gl::TexImage2D(gl::TEXTURE_2D, 
                           0, 
                           format as i32, 
                           image.width() as i32, 
                           image.height() as i32, 
                           0, 
                           format,
                           gl::UNSIGNED_BYTE,
                           (*image.raw()).pixels);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        Ok(texture)
    }*/

    /// Creates a new texture from a path. Only supports RGB formats.
    /// NOTE: May fail with images with alpha data (e.g. PNGs)
    // TODO: Introduce support for alpha data.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let surface = Surface::from_file(&path)?;
        let surface_raw = unsafe {(*surface.raw()).pixels as *const u8};

        let image = match image::open(path) {
            Ok(image) => {
               image 
            },
            Err(e) => return Err(format!("{}", e))
        };
        let (width, height, raw, format) = match image {
            DI::ImageRgb8(image) => {
                let format = gl::RGB;
                let width = image.width();
                let height = image.height();
                let raw = image.as_raw().as_ref() as &[u8];
                let mut counter = 0;
                for p in raw {
                    println!("{}", counter);
                    unsafe {assert_eq!(*p, *surface_raw.offset(counter));}
                    counter += 1;
                }
                let raw = raw.as_ptr() as *const c_void;
                (width, height, raw, format)
            },
            DI::ImageRgba8(image) => {
                let format = gl::RGBA;
                let width = image.width();
                let height = image.height();
                let raw = image.as_raw().as_ref() as &[u8];
                let raw = raw.as_ptr() as *const c_void;
                (width, height, raw, format)
            }
            bad_format => {
                panic!("Cannot understand image format {:?}", bad_format);
            }
        };


        let mut texture = Texture {
            id: 0,
            width: width,
            height: height
        };

        unsafe {
            gl::GenTextures(1, &mut texture.id);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            texture.linear_mipmap_nearest();
            gl::TexImage2D(gl::TEXTURE_2D, 
                           0, 
                           format as i32, 
                           texture.width as i32, 
                           texture.height as i32, 
                           0, 
                           format,
                           gl::UNSIGNED_BYTE,
                           raw);
            gl::GenerateMipmap(gl::TEXTURE_2D);
            println!("Test");
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
