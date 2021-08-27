#version 330 core

out vec4 frag_color;

in vec2 t_pos;
in vec3 v_pos;

uniform sampler2D t_data;

void main() {
    vec4 color = texture(t_data, t_pos);
    if (color.x == 0.0 && color.y == 0.0 && color.z == 0.0)
        color = vec4(v_pos, 1);

    frag_color = color;
}
