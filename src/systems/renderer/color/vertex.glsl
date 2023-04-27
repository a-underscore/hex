#version 330

in vec3 position;
in vec3 normal;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;

void main(void) {
	mat4 view =  transform * inverse(camera_transform);

        vec4 pos = vec4(position, 1.0) * view;

        gl_Position = pos * camera_proj;
}
