use gl;
use std::fs::File;
use std::io::Read;
use std::ffi::CString;
use glam;
use crate::Transform;

// TODO: Be able to pass data so as to correctly calculate aspect ratio.
/// Builder to construct a Shader.
pub struct ShaderBuilder {
    vertex: Option<CString>,
    fragment: Option<CString>,
    perspective: Option<(bool, f32, f32)>
}

impl ShaderBuilder {
    /// Reads the data for the vertex shader.
    pub fn vertex(&mut self, path: &str) -> &mut ShaderBuilder {
        let mut buffer = Vec::new();
        match File::open(path) {
            Ok(mut file) => {
                file.read_to_end(&mut buffer).unwrap();
                self.vertex = Some(CString::new(buffer).unwrap());
                self
            },
            Err(_) => self
        }
    }

    /// Reads the data for the fragment shader.
    pub fn fragment(&mut self, path: &str) -> &mut ShaderBuilder {
        let mut buffer = Vec::new();
        match File::open(path) {
            Ok(mut file) => {
                file.read_to_end(&mut buffer).unwrap();
                self.fragment = Some(CString::new(buffer).unwrap());
                self
            },
            Err(_) => self
        }
    }

    /// Sets the projection type to perspective
    pub fn perspective(&mut self, width: u32, height: u32) -> &mut ShaderBuilder {
        self.perspective = Some((true, width as f32, height as f32));
        self
    }

    /// Sets the projection type to orthographic
    pub fn orthographic(&mut self, width: u32, height: u32) -> &mut ShaderBuilder {
        self.perspective = Some((false, width as f32, height as f32));
        self
    }

    /// Builds the ShaderBuilder into a Shader.
    ///
    /// NOTE: Shader may be enabled after building.
    // TODO: Get proper compilation error messages.
    pub fn build(&mut self) -> Result<Shader, String> {
        if self.vertex.is_none() {
            return Err(String::from("Vertex shader not supplied"));
        }

        if self.fragment.is_none() {
            return Err(String::from("Fragment shader not supplied"));
        }

        let vertex = self.vertex.take().unwrap();
        let fragment = self.fragment.take().unwrap();
        let p_id;
        unsafe {
            let mut success: gl::types::GLint = 1;
            let v_id = gl::CreateShader(gl::VERTEX_SHADER);
            //let f_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(v_id, 1, &vertex.as_ptr(), std::ptr::null());
            gl::CompileShader(v_id);
            gl::GetShaderiv(v_id, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                return Err(String::from("Error: Could not compile vertex shader"));
            }

            let f_id = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(f_id, 1, &fragment.as_ptr(), std::ptr::null());
            gl::CompileShader(f_id);
            gl::GetShaderiv(f_id, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                return Err(String::from("Error: Could not compile fragment shader"));
            }

            p_id = gl::CreateProgram();
            gl::AttachShader(p_id, v_id);
            gl::AttachShader(p_id, f_id);
            gl::LinkProgram(p_id);
            gl::GetProgramiv(p_id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                return Err(String::from("Error: Could not link shader program"));
            }

            gl::DeleteShader(v_id);
            gl::DeleteShader(f_id);

            let (mat, proj_type) = match self.perspective {
                Some((true, width, height)) => {
                    (glam::Mat4::perspective_rh(
                        45.0_f32.to_radians(),
                        width / height,
                        0.1, 
                        100.0,), Some(true))
                },
                Some((false, width, height)) => {
                    (glam::Mat4::orthographic_rh(
                        0.0,
                        width,
                        height,
                        0.0,
                        0.1,
                        100.0), Some(false))
                },
                _ => (glam::Mat4::IDENTITY, None)
            };

            gl::UseProgram(p_id);
            let proj_loc = gl::GetUniformLocation(
                p_id,
                std::ffi::CString::new("proj").unwrap().into_raw());

            gl::UniformMatrix4fv(proj_loc,
                1,
                gl::FALSE,
                mat.as_ref() as *const _);

            let model_loc = gl::GetUniformLocation(
                p_id,
                std::ffi::CString::new("model").unwrap().into_raw());

            Ok(Shader {
                id: p_id,
                proj_loc: proj_loc,
                model_loc: model_loc,
                perspective: proj_type
            })
        }
    }
}

/// A Shader for drawing meshes.
/// NOTE: Only supports an FOV of 45 degrees.
// TODO: Include support for other FOV's
#[derive(Clone, Debug)]
pub struct Shader {
    id: gl::types::GLuint,
    proj_loc: gl::types::GLint,
    model_loc: gl::types::GLint,
    perspective: Option<bool>
}

impl Shader {
    /// Creates a ShaderBuilder to build a Shader from.
    pub fn new() -> ShaderBuilder {
        ShaderBuilder {
            vertex: None,
            fragment: None,
            perspective: None
        }
    }

    /// Enables the Shader.
    pub fn enable(&self) {
        unsafe { gl::UseProgram(self.id); }
    }

    /// Returns the id of the Shader.
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    /// Updates the aspect ratio of the shader. This method
    /// does nothing if no type is assigned.
    pub fn update_aspect_ratio(&self, width: f32, height: f32) {
        match self.perspective {
            Some(is_perspect) => {
                let mat = if is_perspect {
                    glam::Mat4::perspective_rh(
                        45.0_f32.to_radians(),
                        width / height,
                        0.1, 
                        100.0,)
                } else {
                    glam::Mat4::orthographic_rh(
                        0.0,
                        width,
                        height,
                        0.0,
                        0.1,
                        100.0)
                };

                self.enable();
                unsafe {
                    gl::UniformMatrix4fv(self.proj_loc,
                        1,
                        gl::FALSE,
                        mat.as_ref() as *const _);
                }
            },
            None => {}
        };
    }

    pub fn set_transform(&self, transform: &Transform) {
        self.enable();
        unsafe {
            gl::UniformMatrix4fv(self.model_loc,
                                 1,
                                 gl::FALSE,
                                 transform.transformation().as_ref() as *const _);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}
