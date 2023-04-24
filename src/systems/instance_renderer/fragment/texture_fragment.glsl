#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec2 v_uv;
in vec4 v_color;

uniform sampler2D tex;

void main(void) {
	gl_FragColor = texture(tex, v_uv) * v_color;
}
