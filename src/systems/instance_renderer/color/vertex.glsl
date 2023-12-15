#version 330

in mat4 transform;
in vec4 color;
in vec3 position;
in vec3 normal;

out vec4 v_color;

uniform mat4 camera_transform;
uniform mat4 camera_proj;

void main(void) {
        vec3 pos = vec3(transform * vec4(position, 1.0));

        gl_Position = camera_proj * camera_transform * vec4(pos, 1.0);

	v_color = color;
}
