#version 430

#extension GL_ARB_bindless_texture : require

in vec2 tex_pos;

out vec4 frag_color;

layout(std140, binding = 0) uniform Uniforms {
	sampler2D image;
};

uniform vec4 color;

void main() {
	frag_color = texture(image, tex_pos) * color;
}
