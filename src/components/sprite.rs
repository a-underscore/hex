use crate::{
    assets::{Shape, Texture2d},
    ecs::component_manager::Component,
};

#[derive(Clone)]
pub struct Sprite {
    pub shape: Shape,
    pub texture: Texture2d,
    pub color: [f32; 4],
    pub z: f32,
    pub active: bool,
}

impl Sprite {
    pub fn new(shape: Shape, texture: Texture2d, color: [f32; 4], z: f32, active: bool) -> Self {
        Self {
            shape,
            texture,
            color,
            z,
            active,
        }
    }
}

impl Component for Sprite {}
