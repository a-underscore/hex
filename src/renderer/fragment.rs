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
