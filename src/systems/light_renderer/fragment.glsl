#version 330

in vec3 v_pos;
in vec3 v_normal;

uniform sampler2D buffer;
uniform vec2 screen_dims;
uniform vec3 light_color;
uniform vec3 light_pos;
uniform float light_strength;

void main(void) {
	vec3 ambient = light_strength * light_color;
	vec3 light_dir = normalize(light_pos - v_pos);
	vec3 diffuse = max(dot(v_normal, light_dir), 0.0) * light_color;
	vec4 texture = texture(buffer, gl_FragCoord.xy / screen_dims);

	gl_FragColor = vec4(ambient * texture.rgb + diffuse, texture.w);
}
