#version 330

in vec3 position;
in vec3 normal;

uniform mat4 transform;
uniform mat4 light_transform;
uniform mat4 light_proj;

void main(void) {
	mat4 view = transform * inverse(light_transform);

        vec4 pos = vec4(position, 1.0) * view;

        gl_Position = pos * light_proj;
}
