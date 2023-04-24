use crate::{
    assets::{Mesh, Texture},
    ecs::{component_manager::Component, Id},
    id,
    math::Vec4d,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Instance {
    pub data: Rc<(Option<Texture>, Mesh)>,
    pub color: Vec4d,
    pub active: bool,
}

impl Instance {
    pub fn new(mesh: Mesh, texture: Option<Texture>, color: Vec4d, active: bool) -> Self {
        Self {
            data: Rc::new((texture, mesh)),
            color,
            active,
        }
    }
}

impl Component for Instance {
    fn id() -> Id {
        id!()
    }
}
