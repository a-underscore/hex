#version 330

in vec3 position;
in vec3 normal;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;

void main(void) {
	mat4 view = camera_transform * transform;

        vec3 pos = vec3(view * vec4(position, 1.0));

        gl_Position = camera_proj * vec4(pos, 1.0);
}
