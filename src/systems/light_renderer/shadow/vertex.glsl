#version 330

in vec3 position;
in vec3 normal;

out vec3 out_world_pos;
out vec3 out_normal;

uniform mat4 transform;
uniform mat4 light_transform;

void main(void) {
	mat4 view =  inverse(light_transform) * transform;

        vec4 pos = view * vec4(position, 1.0);

	out_world_pos = vec3(pos);

	out_normal = vec3(view * vec4(normal, 1.0));

    	gl_Position = position;
} 
