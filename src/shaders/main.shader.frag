// shader.frag
#version 450

layout(location=0) in vec4 v_color;
layout(location=0) out vec4 f_color;

void main() {
    f_color = vec4(v_color[0]/255,v_color[1]/255,v_color[2]/255,v_color[3]/255);
}
