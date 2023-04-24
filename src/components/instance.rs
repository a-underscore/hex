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
use std::rc::Rc;

#[derive(Clone)]
pub struct Instance<'a> {
    pub draw_parameters: DrawParameters<'a>,
    pub data: Rc<(Option<Texture>, Mesh)>,
    pub color: Vec4d,
    pub active: bool,
}

impl<'a> Instance<'a> {
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
            data: Rc::new((texture, mesh)),
            color,
            active,
        }
    }
}

impl<'a> Component for Instance<'a> {
    fn id() -> Id {
        id!()
    }
}
