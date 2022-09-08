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
        let vertex_src = r#"
            #version 140

            in vec2 position;
            in vec2 uv;

            uniform float z;
            uniform mat3 transform;
            uniform mat3 camera_transform;
            uniform mat4 camera_view;

            out vec2 tex_pos;

            void main() {
                tex_pos = uv;

                vec3 pos = vec3((inverse(camera_transform) * transform * vec3(position, 1.0)).xy, z);

                gl_Position = camera_view * vec4(pos, 1.0);
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
