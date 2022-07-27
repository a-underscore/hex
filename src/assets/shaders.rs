use glium::{Display, Program};
use std::rc::Rc;

pub struct Shaders {
    pub program: Program,
}

impl Shaders {
    pub fn new(program: Program) -> Rc<Self> {
        Rc::new(Self { program })
    }

    pub fn default(display: &Display) -> anyhow::Result<Rc<Self>> {
        let vertex_src = r#"
            #version 140

            in vec2 position;
            in vec2 uv;

            out vec2 tex_pos;

            void main() {
                tex_pos = uv;

                gl_Position =  vec4(position, 0.0, 1.0);
            }
        "#;
        let fragment_src = r#"
            #version 140

            in vec2 tex_pos;

            out vec4 frag_color;

            uniform sampler2D tex;
            uniform vec4 color;

            void main() {
                frag_color = texture(tex, tex_pos) * color;
            }
            
        "#;
        let program = Program::from_source(display, vertex_src, fragment_src, None)?;
        Ok(Self::new(program))
    }
}
