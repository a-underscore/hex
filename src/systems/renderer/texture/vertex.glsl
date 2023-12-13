#version 330

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec2 v_uv;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;

void main(void) {
	mat4 view =  inverse(camera_transform) * transform;

        vec4 pos = view * vec4(position, 1.0);

        gl_Position = camera_proj * pos;

	v_uv = uv;
}
