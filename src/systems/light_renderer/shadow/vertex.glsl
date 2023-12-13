#version 330

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

uniform mat4 wvp;
uniform mat4 world;

out vec3 world_pos;

void main(void) {
	vec4 pos = vec4(position, 1.0);

	gl_Position = wvp * pos;

    	world_pos = (world * pos).xyz;

}
