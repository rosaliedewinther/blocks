// shader.vert
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 a_color;
layout(location=2) in vec3 a_normal;
layout(location=3) in uint type;

layout(location=0) out vec4 v_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 u_view;
    mat4 u_perspective;
    vec3 viewer_pos;
    vec3 sun_dir;
    float time;
};

const vec3 diffuse_color = vec3(1.0, 1.0, 1.0);

void main() {
    vec3 perm_position = a_position;
    if (type == 1){
        float perm = sin(time*5)/2;
        perm_position = perm_position + ((perm_position - (2 * floor(perm_position/2)))-0.5) *perm;
    }
    float diffuse = max(dot(normalize(a_normal), normalize(sun_dir)), 0.1);
    vec4 new_color = vec4(a_color[0]/255,a_color[1]/255,a_color[2]/255,a_color[3]/255);
    v_color = new_color * vec4(diffuse_color * diffuse,1);
    gl_Position = u_perspective * u_view * vec4(perm_position, 1.0);
}
