#version 330 core

layout (location = 0) in vec3 a_v_pos;
layout (location = 1) in vec2 a_t_pos;

out vec2 t_pos;
out vec3 v_pos;

uniform mat4 model = mat4(1);
uniform mat4 proj = mat4(1);

void main() {
    gl_Position = proj * model * vec4(a_v_pos, 1.0);
    t_pos = a_t_pos;
    v_pos = a_v_pos;
}
