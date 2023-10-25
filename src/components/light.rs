use crate::{ecs::component_manager::Component, math::Vec3d};

pub struct Light {
    pub position: Vec3d,
    pub color: Vec3d,
    pub strength: f32,
    pub active: bool,
}

impl Light {
    pub fn new(position: Vec3d, color: Vec3d, strength: f32, active: bool) -> Self {
        Self {
            position,
            color,
            strength,
            active,
        }
    }
}

impl Component for Light {}
