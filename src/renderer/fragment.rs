vulkano_shaders::shader! {
    ty: "fragment",
    src: r"
#version 450

layout(location = 0) in vec2 tex_pos;

layout(location = 0) out vec4 frag_color;

layout(set = 1, binding = 0) uniform sampler s;
layout(set = 1, binding = 1) uniform texture2D tex;
layout(set = 2, binding = 0) uniform Color {
    vec4 color;
};

void main(void) {
	frag_color = texture(sampler2D(tex, s), tex_pos) * color;
}
        ",
}
