#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec2 v_uv;
in vec4 v_color;

out vec4 frag_color;

uniform sampler2D buffer;
uniform vec4 light_color;
uniform float specular;
uniform float diffuse;
uniform float ambient;

void main(void) {
	frag_color = texture(buffer, v_uv) * v_color;
}
