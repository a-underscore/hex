#version 330

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 v_pos;
out vec3 v_normal;
out vec2 v_uv;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_view;

void main(void) {
	mat4 view = camera_transform * transform;

        vec4 pos = vec4(position, 1.0) * view;

        gl_Position = pos * camera_view;

	v_pos = vec3(pos);
	v_normal = normalize(normal * mat3(transpose(inverse(view))));
	v_uv = uv;
}