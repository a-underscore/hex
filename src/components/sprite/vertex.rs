vulkano_shaders::shader! {
    ty: "vertex",
    src: r"
#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;

layout(location = 0) out vec2 tex_pos;

layout(set = 0, binding = 0) uniform View {
    float z;
    mat3 transform;
    mat3 camera_transform;
    mat4 camera_proj;
};

void main(void) {
        vec2 pos = (inverse(camera_transform) * transform * vec3(position, 1.0)).xy;

        gl_Position = camera_proj * vec4(vec3(pos, z), 1.0);

    	tex_pos = uv;
}
        ",
}
