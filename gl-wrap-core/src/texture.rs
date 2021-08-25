use std::path::Path;
use std::ffi::c_void;
use gl::{
    self,
    types::{
        GLuint
    }
};
use image::{open as open_image,
    DynamicImage as DI, 
    GenericImageView,
    ColorType::*
};

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
        let pf = image.pixel_format_enum();
        println!("{:?}", pf);
        let format = match pf {
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
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let mut image = match open_image(path) {
            Ok(image) => {
               image 
            },
            Err(e) => {
                return Err(format!("{}", e))
            }
        };

        // Gather dimensions. Dimensions have to be divisble
        // by 4, so crop if not.
        let (mut width, mut height) = image.dimensions();
        let mut to_crop = false;
        if width % 4 != 0 {
            to_crop = true;
            width = width - 1;
            while width % 4 != 0 {
                width = width - 1;
            }
        }

        if height % 4 != 0 {
            to_crop = true;
            height = height - 1;
            while height % 4 != 0 {
                height = height - 1;
            }
        }

        if to_crop {
            image = image.crop_imm(0, 0, width, height);
        }

        let (format, image) = match image.color() {
            Rgb8 => (gl::RGB, image),
            Rgba8 => (gl::RGBA, image),
            L8 => (gl::RGB, DI::ImageRgb8(image.to_rgb8())),
            La8 => (gl::RGBA, DI::ImageRgba8(image.to_rgba8())),
            _ => panic!("Bad format")
        };
        let raw = image.as_bytes();

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
                           raw.as_ptr() as *const c_void);
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
