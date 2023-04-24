#version 330

in mat4 transform;
in vec3 position;
in vec3 normal;
in vec4 color;

out vec3 v_pos;
out vec3 v_normal;
out vec4 v_color;

uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 view = camera_transform * transform;

        vec4 pos = vec4(position, 1.0) * view;

        gl_Position = pos * camera_view;

	v_pos = vec3(pos);
	v_normal = normalize(normal * mat3(view));
	v_color = color;
}
