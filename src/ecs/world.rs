use super::{component_manager::ComponentManager, entity_manager::EntityManager};
use cgmath::Vector4;
use glium::Display;

pub struct World<'a> {
    pub entity_manager: EntityManager,
    pub component_manager: ComponentManager<'a>,
    pub display: Display,
    pub bg: Vector4<f32>,
}

impl<'a> World<'a> {
    pub fn new(display: Display, bg: Vector4<f32>) -> Self {
        Self {
            entity_manager: EntityManager::default(),
            component_manager: ComponentManager::default(),
            display,
            bg,
        }
    }
}
