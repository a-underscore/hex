use crate::{
    assets::{Mesh, Texture},
    ecs::{component_manager::Component, Id},
    id,
    math::Vec4d,
};

#[derive(Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub texture: Option<Texture>,
    pub color: Vec4d,
    pub active: bool,
}

impl Model {
    pub fn new(mesh: Mesh, texture: Option<Texture>, color: Vec4d, active: bool) -> Self {
        Self {
            mesh,
            texture,
            color,
            active,
        }
    }
}

impl Component for Model {
    fn id() -> Id {
        id!()
    }
}
