use crate::{
    Transform,
    shader::Shader,
    mesh::Mesh,
    texture::Texture
};
use std::rc::Rc;
use std::iter;
use glam::{Vec3, Quat};
use crate::ecs::Entity;
use crate::scriptable::Scriptable;
use itertools::izip;

pub struct Material;

pub trait Component {
    fn create(_entity: &mut Entity) -> Self;
    fn update(&mut self) {}
}

pub struct MeshComponent {
    meshes: Vec<Rc<Mesh>>,
    textures: Vec<Rc<Texture>>,
    shaders: Vec<Rc<Shader>>,
    transforms: Vec<Transform>,
}

impl MeshComponent {
    // TODO: Make this more refined, don't mass change
    pub fn set_shader(&mut self, new_shader: Rc<Shader>) {
        self.shaders = self.shaders.splice(
            ..self.shaders.len(), 
            iter::once(new_shader).cycle())
            .collect();
    }

    pub fn set_translate(&mut self, fx: f32, fy: f32, fz: f32) {
        for transform in &mut self.transforms {
            transform.translation.x = fx;
            transform.translation.y = fy;
            transform.translation.z = fz;
        }
    }

    pub fn set_scale(&mut self, fx: f32, fy: f32, fz: f32) {
        for transform in &mut self.transforms {
            transform.scale.x = fx;
            transform.scale.y = fy;
            transform.scale.z = fz;
        }
    }

    pub fn set_rotation(&mut self, axis: Vec3, angle: f32) {
        for transform in &mut self.transforms {
            transform.rotation = Quat::from_axis_angle(axis, angle);
        }
    }
    
    pub fn add_mstm(&mut self, 
                    mesh: Rc<Mesh>, 
                    shader: Rc<Shader>, 
                    texture: Option<Rc<Texture>>,
                    _material: Option<Material>) {
        self.meshes.push(mesh);
        self.shaders.push(shader);
        if !texture.is_none() {
            self.textures.push(texture.unwrap());
        }
        /*if !material.is_none() {
            self.materials.push(texture.unwrap());
        }*/
        self.transforms.push(Transform::new());
    }

    pub fn textures(&self) -> &[Rc<Texture>]{
        &self.textures
    }
}

impl Component for MeshComponent {
    fn create(_entity: &mut Entity) -> Self {
        MeshComponent {
            meshes: Vec::new(),
            textures: Vec::new(),
            shaders: Vec::new(),
            transforms: Vec::new(),
        }
    }

    // TODO: Test for unusual cases where vecs are not the same size
    // TODO: Use textures
    fn update(&mut self) {
        let draw = izip!(
            self.meshes.iter(),
            self.textures.iter(),
            self.shaders.iter(),
            self.transforms.iter()
        );

        for (mesh, texture, shader, transform) in draw {
            shader.set_transform(&transform);
            texture.enable();
            mesh.draw();
        }
    }
}

#[derive(Debug)]
pub struct ScriptComponent<T: Scriptable> {
    script: T,
    entity: *mut Entity
}

impl<T: Scriptable> Component for ScriptComponent<T> {
    fn create(entity: &mut Entity) -> Self {
        let mut comp = ScriptComponent {
            script: T::create(),
            entity: entity as *mut _
        };
        unsafe {
            comp.script.on_create(&mut *comp.entity as &mut _);
        }
        comp
    }

    fn update(&mut self) {
        unsafe {
            self.script.on_update(&mut *self.entity as &mut _);
        }
    }
}

pub mod components {
    pub use crate::component::{
        MeshComponent,
        ScriptComponent
    };
}