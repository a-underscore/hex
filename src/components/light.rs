use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::Vec3d,
};

pub struct Light {
    pub color: Vec3d,
    pub strength: f32,
    pub active: bool,
}

impl Light {
    pub fn new(color: Vec3d, strength: f32, active: bool) -> Self {
        Self {
            color,
            strength,
            active,
        }
    }
}

impl Component for Light {
    fn id() -> Id {
        id!()
    }
}
