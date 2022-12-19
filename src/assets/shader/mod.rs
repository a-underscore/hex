use glium::{Display, Program};
use std::rc::Rc;

pub static VERTEX_SRC: &str = include_str!("vertex.vsh");
pub static FRAGMENT_SRC: &str = include_str!("fragment.fsh");

pub struct Shader {
    pub program: Program,
}

impl Shader {
    pub fn new(program: Program) -> Rc<Self> {
        Rc::new(Self { program })
    }

    pub fn default(display: &Display) -> anyhow::Result<Rc<Self>> {
        let program = Program::from_source(display, VERTEX_SRC, FRAGMENT_SRC, None)?;

        Ok(Self::new(program))
    }
}
