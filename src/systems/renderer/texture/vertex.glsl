#version 330

in vec3 position;
in vec3 normal;
in vec2 uv;

out vec3 v_pos;
out vec2 v_uv;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;

void main(void) {
	mat4 model =  transform * inverse(camera_transform);

        vec4 pos = vec4(position, 1.0) * model;

        gl_Position = pos * camera_proj;

	v_pos = vec3(pos);
	v_uv = uv;
}