#version 330

in vec2 position;
in vec2 uv;

out vec2 tex_pos;

uniform float z;
uniform mat3 transform;
uniform mat3 camera_transform;
uniform mat4 camera_view;

void main(void) {
	tex_pos = uv;

        vec3 pos = vec3((inverse(camera_transform) * transform * vec3(position, 1.0)).xy, z);

        gl_Position = camera_view * vec4(pos, 1.0);
}
