#version 330

layout (location = 0) in vec3 pos;
layout (location = 1) in vec2 tex_coords;
layout (location = 2) in vec3 normal;

uniform mat4 wvp;
uniform mat4 world;

out vec3 world_pos;

void main(void) {
	vec4 pos4 = vec4(pos, 1.0);

	gl_Position = wvp * pos4;

    	world_pos = (world * pos4).xyz;

}
