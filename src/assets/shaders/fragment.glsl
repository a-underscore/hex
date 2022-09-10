#version 140

in vec2 tex_pos;

out vec4 frag_color;

uniform sampler2D tex;
uniform vec4 color;

void main() {
	frag_color = texture(tex, tex_pos) * color;
}
