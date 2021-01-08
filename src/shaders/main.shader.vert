// shader.vert
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 a_color;
layout(location=2) in vec3 a_normal;

layout(location=0) out vec4 v_color;

layout(set=0, binding=0) // 1.
uniform Uniforms {
    mat4 u_view; // 2.
    mat4 u_perspective; // 2.
    vec3 viewer_pos;
};

const vec3 diffuse_color = vec3(1.0, 1.0, 1.0);

void main() {

    float diffuse = max(dot(normalize(a_normal), normalize(viewer_pos)), 0.1);
    vec4 new_color = vec4(a_color[0]/255,a_color[1]/255,a_color[2]/255,a_color[3]/255);
    v_color = new_color * vec4(diffuse_color * diffuse,1);
    gl_Position = u_perspective * u_view * vec4(a_position, 1.0);
}
