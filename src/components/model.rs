use crate::{
    assets::{Mesh, Texture},
    ecs::{component_manager::Component, Id},
    id,
    math::Vec4d,
};
use glium::{
    draw_parameters::{BackfaceCullingMode, Blend, DepthTest},
    Depth, DrawParameters,
};

#[derive(Clone)]
pub struct Model<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub mesh: Mesh,
    pub texture: Option<Texture>,
    pub color: Vec4d,
    pub active: bool,
}

impl<'a> Model<'a> {
    pub fn new(mesh: Mesh, texture: Option<Texture>, color: Vec4d, active: bool) -> Self {
        Self {
            draw_parameters: DrawParameters {
                depth: Depth {
                    test: DepthTest::IfLessOrEqual,
                    write: true,
                    ..Default::default()
                },
                blend: Blend::alpha_blending(),
                backface_culling: BackfaceCullingMode::CullClockwise,
                ..Default::default()
            },
            mesh,
            texture,
            color,
            active,
        }
    }
}

impl<'a> Component for Model<'a> {
    fn id() -> Id {
        id!()
    }
}
