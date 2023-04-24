#version 330

in vec3 v_pos;
in vec3 v_normal;
om vec4 color;

out vec4 frag_color;

void main(void) {
	frag_color = color;
}
