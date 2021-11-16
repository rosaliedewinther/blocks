// shader.vert
#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in uint a_material;
layout(location=2) in uint a_normal;

layout(location=0) out vec4 v_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 u_view;
    mat4 u_perspective;
    vec3 viewer_pos;
    float time;
    vec3 sun_dir;
    float _padding;
    vec4 materials[16];
};

const vec3[] normals = vec3[6](
    vec3(1.0,0.0,0.0), //right
    vec3(-1.0,0.0,0.0),//left
    vec3(0.0,1.0,0.0), //up
    vec3(0.0,-1.0,0.0),//down
    vec3(0.0,0.0,1.0), //front
    vec3(0.0,0.0,-1.0) //back
);

const vec3 diffuse_color = vec3(1.0, 1.0, 1.0);

void main() {
    vec3 perm_position = a_position;
    if (a_material == 6){
        float perm_x = cos(perm_position[0]+time)/4;
        float perm_y = cos(perm_position[1]+time)/4;
        float perm_z = cos(perm_position[2]+time)/4;
        vec3 permutation = vec3(perm_x, perm_y, perm_z);
        perm_position = perm_position + permutation;
    }
    float diffuse = max(dot(normalize(normals[a_normal]), normalize(sun_dir)), 0.1);
    vec4 new_color = vec4(materials[a_material][0]/255,materials[a_material][1]/255,materials[a_material][2]/255,materials[a_material][3]/255);
    v_color = new_color * vec4(diffuse_color * diffuse,1);
    gl_Position = u_perspective * u_view * vec4(perm_position, 1.0);
}
