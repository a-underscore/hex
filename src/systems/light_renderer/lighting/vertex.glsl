#version 330

in vec3 position;
in vec2 tex_coord;
in vec3 normal;

out vec2 out_tex_coord;
out vec3 out_normal;
out vec3 out_world_pos;

uniform mat4 wvp;
uniform mat4 world;

void main(void) {
    	gl_Position = wvp * vec4(position, 1.0);

    	out_tex_coord = tex_coord;

    	out_normal = (world * vec4(normal, 0.0)).xyz;

    	out_world_pos = (world * vec4(position, 1.0)).xyz;
} 
