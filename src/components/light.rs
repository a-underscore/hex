use crate::{assets::Proj, ecs::component_manager::Component};
use cgmath::Vector3;

pub struct Light {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub proj: Proj,
    pub strength: f32,
    pub active: bool,
}

impl Light {
    pub fn new(
        position: Vector3<f32>,
        color: Vector3<f32>,
        proj: Proj,
        strength: f32,
        active: bool,
    ) -> Self {
        Self {
            position,
            color,
            proj,
            strength,
            active,
        }
    }
}

impl Component for Light {}
