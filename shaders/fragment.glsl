#version 330 core

out vec4 frag_color;

in vec2 t_pos;

uniform sampler2D t_data;

void main() {
    frag_color = texture(t_data, t_pos);
}
