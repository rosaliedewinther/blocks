// shader.frag
#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(rgba8ui, binding = 0) readonly uniform uimage2D img_output;

void main() {
    vec2 size = imageSize(img_output);
    f_color = vec4(imageLoad(img_output, ivec2(v_tex_coords*size)))/255;
}
