use glium::{program::Program, Display};
use std::rc::Rc;

#[derive(Clone)]
pub struct Shader {
    pub program: Rc<Program>,
}

impl Shader {
    pub fn new(
        display: &Display,
        vertex_src: &str,
        fragment_src: &str,
        geometry_src: Option<&str>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            program: Rc::new(Program::from_source(
                display,
                vertex_src,
                fragment_src,
                geometry_src,
            )?),
        })
    }
}
