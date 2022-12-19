use glium::{Display, Program};
use std::rc::Rc;

pub static VERTEX_SRC: &str = include_str!("vertex.vsh");
pub static FRAGMENT_SRC: &str = include_str!("fragment.fsh");

#[derive(Clone)]
pub struct Shader {
    pub program: Rc<Program>,
}

impl Shader {
    pub fn new(display: &Display) -> anyhow::Result<Self> {
        let program = Rc::new(Program::from_source(
            display,
            VERTEX_SRC,
            FRAGMENT_SRC,
            None,
        )?);

        Ok(Self { program })
    }
}
