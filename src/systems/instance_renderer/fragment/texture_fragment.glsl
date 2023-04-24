#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec2 tex_pos;
in vec4 color;

out vec4 frag_color;

uniform sampler2D tex;

void main(void) {
	frag_color = texture(tex, tex_pos) * color;
}
