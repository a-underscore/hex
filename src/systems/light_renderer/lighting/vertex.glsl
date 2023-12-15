#version 330

in vec3 position;
in vec3 normal;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;

out vec3 v_pos;
out vec3 v_normal;

void main(void) {
        vec3 pos = vec3(transform * vec4(position, 1.0));

        gl_Position = camera_proj * camera_transform * vec4(pos, 1.0);

	v_pos = pos;
	v_normal = normalize(mat3(transpose(inverse(transform))) * normal);
}
