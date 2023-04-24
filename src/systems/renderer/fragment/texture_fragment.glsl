#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec2 v_uv;

uniform sampler2D tex;
uniform vec4 color;

void main(void) {
	gl_FragColor = texture(tex, v_uv) * color;
}
