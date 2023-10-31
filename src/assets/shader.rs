use std::sync::Arc;
use vulkano::{device::Device, shader::ShaderModule};

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 tex_pos;

layout(set = 0, binding = 0) uniform View {
    float z;
    mat3 transform;
    mat3 camera_transform;
    mat4 camera_proj;
};

void main(void) {
        vec2 pos = (vec3(position, 1.0) * transform * inverse(camera_transform)).xy;

        gl_Position = vec4(vec3(pos, z), 1.0) * camera_proj;

	tex_pos = uv;
}
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
#version 450

layout(location = 0) in vec2 tex_pos;

layout(location = 0) out vec4 frag_color;

layout(set = 1, binding = 0) uniform Color {
    vec4 color;
};
layout(set = 1, binding = 1) uniform sampler2D tex;

void main(void) {
	frag_color = texture(tex, tex_pos) * color;
}
        ",
    }
}

#[derive(Clone)]
pub struct Shader {
    pub vertex: Arc<ShaderModule>,
    pub fragment: Arc<ShaderModule>,
}

impl Shader {
    pub fn new(device: Arc<Device>) -> anyhow::Result<Self> {
        Ok(Self {
            vertex: vs::load(device.clone())?,
            fragment: fs::load(device.clone())?,
        })
    }
}
