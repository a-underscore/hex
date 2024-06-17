use crate::{
    assets::{Shape, Texture},
    ecs::component_manager::Component,
};
use nalgebra::Vector4;

#[derive(Clone)]
pub struct Sprite {
    pub shape: Shape,
    pub texture: Texture,
    pub color: Vector4<f32>,
    pub layer: i32,
    pub active: bool,
}

impl Sprite {
    pub fn new(
        shape: Shape,
        texture: Texture,
        color: Vector4<f32>,
        layer: i32,
        active: bool,
    ) -> Self {
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
