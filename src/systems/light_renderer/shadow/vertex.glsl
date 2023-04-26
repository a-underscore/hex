#version 330

in vec3 position;
in vec3 normal;

out vec3 v_pos;

uniform mat4 transform;
uniform mat4 light_transform;
uniform mat4 light_view;

void main(void) {
	mat4 model = transform * inverse(light_transform);

        vec4 pos = vec4(position, 1.0) * model;

        gl_Position = pos * light_view;
}
