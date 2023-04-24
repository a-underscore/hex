use crate::{
    ecs::{component_manager::Component, Id},
    id,
    math::Vec4d,
};

pub struct Light {
    pub color: Vec4d,
    pub specular: f32,
    pub diffuse: f32,
    pub ambient: f32,
    pub active: bool,
}

impl Component for Light {
    fn id() -> Id {
        id!()
    }
}
