pub mod uniforms;

pub use uniforms::Uniforms;

use glium::{Display, Program};
use std::{cell::RefCell, rc::Rc};

pub struct Shaders {
    pub program: Program,
}

impl Shaders {
    pub fn new(program: Program) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { program }))
    }

    pub fn new_default(display: &Display) -> anyhow::Result<Rc<RefCell<Self>>> {
        let vertex_src = include_str!("vertex.vsh");
        let fragment_src = include_str!("fragment.fsh");
        let program = Program::from_source(display, vertex_src, fragment_src, None)?;

        Ok(Self::new(program))
    }
}
