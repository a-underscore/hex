use crate::{
    assets::{Shape, Texture},
    cid,
    ecs::component_manager::Component,
};
use glium::{
    draw_parameters::{Blend, DepthTest},
    Depth, DrawParameters,
};

#[derive(Clone)]
pub struct Sprite<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub shape: Shape,
    pub texture: Texture,
    pub color: [f32; 4],
    pub z: f32,
    pub active: bool,
}

impl<'a> Sprite<'a> {
    pub fn new(shape: Shape, texture: Texture, color: [f32; 4], z: f32, active: bool) -> Self {
        Self {
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                ..Default::default()
            },
            shape,
            texture,
            color,
            z,
            active,
        }
    }
}

impl<'a> Component for Sprite<'a> {
    fn id() -> usize {
        cid!()
    }
}
