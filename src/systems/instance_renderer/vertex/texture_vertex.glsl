#version 330

in mat4 transform;
in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 uv;

out vec3 v_pos;
out vec3 v_normal;
out vec2 v_uv;
out vec4 v_color;

uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 view = transform * inverse(camera_transform);
        vec4 pos = vec4(position, 1.0) * view;
	vec4 normal = vec4(normal, 1.0) * view;

        gl_Position = pos * camera_view;

	v_pos = vec3(pos);
	v_normal = vec3(normal);
	v_uv = uv;
	v_color = color;
}
