use crate::ecs::{self, Type};
use glium::{Display, Program};

pub const VERTEX_SRC: &str = include_str!("vertex.vsh");
pub const FRAGMENT_SRC: &str = include_str!("fragment.fsh");

pub struct Shaders {
    pub program: Program,
}

impl Shaders {
    pub fn new(program: Program) -> Type<Self> {
        ecs::new(Self { program })
    }

    pub fn new_default(display: &Display) -> anyhow::Result<Type<Self>> {
        let program = Program::from_source(display, VERTEX_SRC, FRAGMENT_SRC, None)?;

        Ok(Self::new(program))
    }
}
