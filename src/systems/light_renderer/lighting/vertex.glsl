#version 330

in vec3 position;
in vec3 normal;

out vec3 v_pos;
out vec3 v_normal;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 camera_proj;
uniform mat4 light_proj;

void main(void) {
	mat4 model =  transform * inverse(camera_transform);

        vec4 pos = vec4(position, 1.0) * model;

        gl_Position = pos * camera_proj;

	v_pos = vec3(pos);
	v_normal = normalize(normal * mat3(transpose(inverse(model))));
}
