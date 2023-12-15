#version 330

in vec3 position;
in vec3 normal;

out vec3 out_world_pos;

uniform mat4 transform;
uniform mat4 light_transform;
uniform mat4 light_proj;

void main(void) {
	mat4 view = light_transform * transform;

        vec3 pos = vec3(view * vec4(position, 1.0));

    	gl_Position = light_proj * vec4(pos, 1.0);

	out_world_pos = vec3(pos);
} 
