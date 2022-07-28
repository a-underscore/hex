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

            uniform mat3 translation;
            uniform mat2 rotation;
            uniform mat3 scale;
            uniform float z;
            uniform mat4 camera_view;
            uniform mat3 camera_translation;
            uniform mat2 camera_rotation;

            out vec2 tex_pos;

            void main() {
                tex_pos = uv;

                mat3 global_transform = (inverse(translation) * mat3(rotation)) * (mat3(camera_rotation) * camera_translation);

                vec3 pos = vec3((global_transform * scale * vec3(position, 1.0)).xy, z);

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
