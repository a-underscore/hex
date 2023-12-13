#version 330

in vec3 pos;

uniform vec3 light_pos;

out float frag_color;

uniform 
void main(void) {
	vec3 light_to_vertex = pos - light_pos;

	float light_to_pixel_distance = length(light_to_vertex);

	frag_color = light_to_pixel_distance;
}
