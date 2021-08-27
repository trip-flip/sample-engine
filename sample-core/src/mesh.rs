use gl::{
    self,
    types::{
        GLfloat,
        GLuint,
    }
};

/// Builder for a Mesh
pub struct MeshBuilder {
    name: String,
    vertices: Vec<u8>,
    indices: Vec<u8>,
    uv: Vec<GLfloat>,
}

impl MeshBuilder {
    pub fn name<S: Into<String>>(mut self, v: S) -> MeshBuilder {
        self.name = v.into();
        self
    }

    /// Add vertices to MeshBuilder.
    pub fn vertices(mut self, v: Vec<u8>) -> MeshBuilder {
        self.vertices = v;
        self
    }

    /// Add indices to MeshBuilder.
    pub fn indices(mut self, i: Vec<u8>) -> MeshBuilder {
        self.indices = i;
        self
    }

    /// Add UVs to MeshBuilder.
    pub fn uv(mut self, u: Vec<GLfloat>) -> MeshBuilder {
        self.uv = u;
        self
    }

    /// Converts the MeshBuilder into a Mesh.
    ///
    /// If vertex, index, or shader data is not supplied,
    /// the method errors.
    pub fn build(self) -> Result<Mesh, String> {
        if self.vertices.is_empty() {
            return Err(String::from("Error: Did not supply vertices"));
        }

        if self.indices.is_empty() {
            return Err(String::from("Error: Did not supply indices"));
        }

        if self.name.is_empty() {
            return Err(String::from("Error: Did not supply name"));
        }

        let mut mesh = Mesh {
            vao: 0,
            vbo: 0,
            ebo: 0,
            uv: 0,
            index_count: (self.indices.len() / std::mem::size_of::<u16>()) as _,
        };
        
        unsafe {
            gl::GenVertexArrays(1, &mut mesh.vao);
            gl::BindVertexArray(mesh.vao);

            gl::GenBuffers(1, &mut mesh.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, 
                           //(self.vertices.len() /** std::mem::size_of::<f32>()*/) as gl::types::GLsizeiptr,
                           (self.vertices.len()) as gl::types::GLsizeiptr,
                           self.vertices.as_ptr() as *const _, 
                           gl::STATIC_DRAW);

            gl::GenBuffers(1, &mut mesh.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 
                           //(self.indices.len() * std::mem::size_of::<u16>()) as gl::types::GLsizeiptr,
                           (self.indices.len()) as gl::types::GLsizeiptr,
                           self.indices.as_ptr() as *const _, 
                           gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, 
                                    gl::FLOAT, 
                                    gl::FALSE, 
                                    //(3 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                    0,
                                    std::ptr::null());
            gl::EnableVertexAttribArray(0);

            if !self.uv.is_empty() {
                gl::GenBuffers(1, &mut mesh.uv);
                gl::BindBuffer(gl::ARRAY_BUFFER, mesh.uv);
                gl::BufferData(gl::ARRAY_BUFFER, 
                               (self.uv.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                               self.uv.as_ptr() as *const _, 
                               gl::STATIC_DRAW);
                gl::VertexAttribPointer(1, 2, 
                                        gl::FLOAT, 
                                        gl::FALSE, 
                                        (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                        std::ptr::null());
                gl::EnableVertexAttribArray(1);
            }

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Ok(mesh)
    }
}

/// The Mesh representation.
#[derive(Debug)]
pub struct Mesh {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: GLuint,
    pub uv: GLuint,

    pub index_count: i32,
}

impl Mesh {
    /// Create a builder for the Mesh.
    pub fn new() -> MeshBuilder {
        MeshBuilder {
            name: String::new(),
            vertices: Vec::new(),
            indices: Vec::new(),
            uv: Vec::new(),
        }
    }

    /// Draw the Mesh.
    pub fn draw(&self) {
        unsafe { 
            gl::BindVertexArray(self.vao);
            gl::Enable(gl::DEPTH_TEST);
            gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_SHORT, std::ptr::null());
        }
    }

}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &mut self.vao);
            gl::DeleteBuffers(1, &mut self.vbo); 
            gl::DeleteBuffers(1, &mut self.ebo); 
        }
    }
}

