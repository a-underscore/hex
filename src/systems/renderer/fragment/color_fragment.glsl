#version 330

in vec3 v_pos;
in vec3 v_normal;

out vec4 frag_color;

uniform vec4 color;

void main(void) {
	frag_color = color;
}
