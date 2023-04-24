#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec4 v_color;

void main(void) {
	gl_FragColor = v_color;
}
