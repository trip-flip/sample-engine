use crate::{
    Transform,
    mesh::Mesh,
    component::MeshComponent,
    shader::Shader
};
use std::rc::Rc;
use glam::Vec3;

#[allow(dead_code)]
pub struct Model {
    meshes: Vec<MeshComponent>,
    transform: Transform,

    script: Option<Box<dyn Fn()>>
}

impl Model {
    pub fn from_mesh(mesh: Rc<Mesh>, shader: Shader) -> Model {
        let mut meshes = Vec::new();
        meshes.push(MeshComponent::new(mesh, shader));
        Model {
            meshes: meshes,
            transform: Transform::new(),
            script: None
        }
    }

    pub fn draw(&self) {
        for mesh in &self.meshes {
            mesh.draw();
        }
    }

    pub fn set_translate(&mut self, fx: f32, fy: f32, fz: f32) {
        for mesh in &mut self.meshes {
            mesh.set_translate(fx, fy, fz);
        }
    }

    pub fn set_scale(&mut self, fx: f32, fy: f32, fz: f32) {
        for mesh in &mut self.meshes {
            mesh.set_scale(fx, fy, fz);
        }
    }

    pub fn set_rotation(&mut self, axis: Vec3, angle: f32) {
        for mesh in &mut self.meshes {
            mesh.set_rotation(axis, angle);
        }
    }

    pub fn play(&self) {
        match &self.script {
            Some(script) => {
                script();
            },
            None => {}
        };
    }
}
