#version 330

in vec3 position;

uniform vec3 light_pos;

out float frag_color;

void main(void) {
	vec3 light_to_vertex = position - light_pos;

	float light_to_pixel_distance = length(light_to_vertex);

	frag_color = light_to_pixel_distance;
}
