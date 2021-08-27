// TODO: Figure out efficient way to create C-like string without allocation.

pub mod shader;
pub mod texture;
pub mod mesh;
pub mod component;
pub mod ecs;
pub mod scriptable;
pub mod scripting;
pub mod assets;

//mod instance;
mod types;

use glam::{
    Vec3,
    Quat
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE
        }
    }
    
    pub fn transformation(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(
            self.scale,
            self.rotation,
            self.translation)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
