use crate::{
    assets::{Material, Mesh, Texture},
    ecs::component_manager::Component,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Model {
    pub data: Rc<(Mesh, Material, Option<Texture>)>,
    pub active: bool,
}

impl Model {
    pub fn new(mesh: Mesh, material: Material, texture: Option<Texture>, active: bool) -> Self {
        Self {
            data: Rc::new((mesh, material, texture)),
            active,
        }
    }
}

impl Component for Model {}
