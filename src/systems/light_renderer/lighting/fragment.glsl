#version 330

in vec3 v_pos;
in vec3 v_normal;
in vec4 v_shadow;

uniform sampler2D buffer;
uniform sampler2DShadow shadow_buffer;
uniform vec3 camera_position;
uniform vec3 light_color;
uniform vec3 light_position;
uniform vec2 screen_dims;
uniform float light_strength;
uniform float ambient_strength;
uniform float specular_strength;
uniform float diffuse_strength;
uniform float reflect_strength;
uniform float bias;

vec3 ambient(void);
vec3 diffuse(vec3);
vec3 specular(vec3);
float shadow(void);

void main(void) {
	vec4 texture = texture(buffer, gl_FragCoord.xy / screen_dims);
	vec3 a = ambient();
	vec3 light_dir = normalize(light_position - v_pos);
	vec3 d = diffuse(light_dir);
	vec3 s = specular(light_dir);
	float shadow = shadow();

	gl_FragColor = vec4(shadow * light_strength * texture.xyz * (d + s) + a, texture.w);
}

vec3 ambient(void) {
	return ambient_strength * light_color;
}

vec3 diffuse(vec3 light_dir) {
	return max(dot(v_normal, light_dir), 0.0) * light_color;
}

vec3 specular(vec3 light_dir) {
	vec3 camera_dir = normalize(camera_position - v_pos);
	vec3 reflect_dir = reflect(-light_dir, v_normal);

	float spec = pow(max(dot(camera_dir, reflect_dir), 0.0), reflect_strength);

	return specular_strength * spec * light_color;
}

float shadow(void) {
	float vis = texture(shadow_buffer, vec3(v_shadow.xy, (v_shadow.z - bias) / v_shadow.w));

	return vis;
}
