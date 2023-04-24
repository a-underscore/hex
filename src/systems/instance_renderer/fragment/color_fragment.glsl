#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec4 v_color;

out vec4 frag_color;

void main(void) {
	frag_color = v_color;
}
