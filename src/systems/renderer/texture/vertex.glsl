#version 330

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec2 v_uv;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;

void main(void) {
        vec3 pos = vec3(transform * vec4(position, 1.0));

        gl_Position = camera_proj * camera_transform * vec4(pos, 1.0);

	v_uv = uv;
}
