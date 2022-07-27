use crate::{assets::Shaders, ecs::Entity};
use cgmath::Vector4;
use std::rc::Rc;

pub struct Scene {
    pub root: Rc<Entity>,
    pub shaders: Rc<Shaders>,
    pub bg: Vector4<f32>,
}

impl Scene {
    pub fn new(root: Rc<Entity>, shaders: Rc<Shaders>, bg: Vector4<f32>) -> Rc<Self> {
        Rc::new(Self { root, shaders, bg })
    }
}
