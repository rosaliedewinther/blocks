// shader.frag
#version 450

// Changed
layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

// NEW!
//layout(binding = 0) uniform texture2D t_diffuse;
layout(binding = 0) uniform usampler2D s_diffuse;


void main() {
    // Changed
    f_color = texture(s_diffuse, v_tex_coords);
}
