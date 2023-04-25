#version 330

in vec3 v_pos;

uniform vec4 color;

void main(void) {
	gl_FragColor = color;
}
