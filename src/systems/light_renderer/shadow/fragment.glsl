#version 330

in vec3 position;

uniform vec3 light_position;

void main(void) {
	vec3 light_to_vertex = position - light_position;

	float light_to_pixel_distance = length(light_to_vertex);

	gl_FragDepth = light_to_pixel_distance;
}
