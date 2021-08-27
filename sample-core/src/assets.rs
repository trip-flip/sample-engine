use std::rc::Rc;
use std::collections::HashMap;
use std::path::Path;
use crate::{
    mesh::Mesh,
    shader::Shader,
    texture::Texture,
};
pub struct Assets {
    meshes: HashMap<String, Rc<Mesh>>,
    shaders: HashMap<String, Rc<Shader>>,
    textures: HashMap<String, Rc<Texture>>,
    //materials: Vec<Rc<Material>>,
    //models: Vec<Rc<Model>>
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            meshes: HashMap::new(),
            shaders: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn import<P: AsRef<Path>>(path: P) {
        unimplemented!();
    }

    pub fn get_mesh(&self, name: &str) -> Option<Rc<Mesh>> {
        unimplemented!();
    }
}