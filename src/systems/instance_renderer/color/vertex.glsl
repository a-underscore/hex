#version 330

in mat4 transform;
in vec4 color;
in vec3 position;
in vec3 normal;

out vec4 v_color;

uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 model =  transform * inverse(camera_transform);

        vec4 pos = vec4(position, 1.0) * model;

        gl_Position = pos * camera_view;

	v_color = color;
}
