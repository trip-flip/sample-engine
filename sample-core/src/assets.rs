use std::rc::Rc;
use crate::{
    mesh::Mesh,
    shader::Shader,
    texture::Texture,
};
pub struct Assets {
    meshes: Vec<Rc<Mesh>>,
    shaders: Vec<Rc<Shader>>,
    textures: Vec<Rc<Texture>>,
    //materials: Vec<Rc<Material>>,
    //models: Vec<Rc<Model>>
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            meshes: Vec::new(),
            shaders: Vec::new(),
            textures: Vec::new(),
        }
    }
}