use crate::{
    assets::{Shape, Texture},
    ecs::component_manager::Component,
};

#[derive(Clone)]
pub struct Sprite {
    pub shape: Shape,
    pub texture: Texture,
    pub color: [f32; 4],
    pub layer: u32,
    pub active: bool,
}

impl Sprite {
    pub fn new(shape: Shape, texture: Texture, color: [f32; 4], layer: u32, active: bool) -> Self {
        Self {
            shape,
            texture,
            color,
            layer,
            active,
        }
    }
}

impl Component for Sprite {}
