#version 330

in vec3 position;
in vec3 normal;

out vec3 v_pos;
out vec3 v_normal;
out vec4 v_shadow;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;
uniform mat4 light_transform;
uniform mat4 light_proj;

void main(void) {
	mat4 view = transform * inverse(camera_transform);

        vec4 pos = vec4(position, 1.0) * view;

        gl_Position = pos * camera_proj;

	v_pos = vec3(vec4(position, 1.0) * transform);
	v_normal = normalize(normal * mat3(transpose(inverse(transform))));

	mat4 depth = transform * inverse(light_transform);

	vec4 shadow_pos = vec4(position, 1.0) * depth;

	v_shadow = shadow_pos * light_proj;
}
