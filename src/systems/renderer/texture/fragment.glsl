#version 330

in vec2 v_uv;

uniform sampler2D buffer;
uniform vec4 color;

void main(void) {
	gl_FragColor = texture(buffer, v_uv) * color;
}
