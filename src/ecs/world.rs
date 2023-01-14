use super::{component_manager::ComponentManager, entity_manager::EntityManager};
use cgmath::Vector4;
use glium::Display;

pub struct World<'a> {
    pub em: EntityManager,
    pub cm: ComponentManager<'a>,
    pub display: Display,
    pub bg: Vector4<f32>,
}

impl<'a> World<'a> {
    pub fn new(display: Display, bg: Vector4<f32>) -> Self {
        Self {
            em: EntityManager::default(),
            cm: ComponentManager::default(),
            display,
            bg,
        }
    }
}
