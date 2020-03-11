#version 140

in vec2 texture_coords;

uniform sampler2D u_texture;

out vec4 out_color;

void main() {
    out_color = texture(u_texture, texture_coords);
}
