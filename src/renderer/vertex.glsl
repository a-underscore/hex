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
        vec3 pos = vec3(vec4(position, 1.0) * view);
	vec3 normal = vec3(vec4(normal, 1.0) * view);

        gl_Position = vec4(pos, 1.0) * camera_view;

	v_pos = pos;
	v_normal = normal;
	tex_pos = uv;
}
