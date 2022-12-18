use glium::{Display, Program};
use std::rc::Rc;

pub static VERTEX_SRC: &str = include_str!("vertex.vsh");
pub static FRAGMENT_SRC: &str = include_str!("fragment.fsh");

pub struct Shader {
    pub program: Program,
}

impl Shader {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn default(display: &Rc<Display>) -> anyhow::Result<Self> {
        let program = Program::from_source(display.as_ref(), VERTEX_SRC, FRAGMENT_SRC, None)?;

        Ok(Self::new(program))
    }
}
