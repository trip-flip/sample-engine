use crate::ecs::Entity;

pub trait Scriptable {
    fn create() -> Self;
    fn on_create() {}
    fn on_update(&mut self, _entity: *mut Entity) {}
    fn on_destroy() {}
}
