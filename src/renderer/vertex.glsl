#version 330

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 v_pos;
out vec3 v_normal;
out vec2 tex_pos;

uniform float z;
uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 view = transform * camera_transform;
        vec4 pos = vec4(position, 1.0) * view;
	vec4 normal = vec4(normal, 1.0) * view;

        gl_Position = pos * camera_view;

	v_pos = vec3(pos);
	v_normal = vec3(normal);
	tex_pos = uv;
}
