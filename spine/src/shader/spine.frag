#version 140

in vec2 v_texCoords;

uniform sampler2D u_texture;

out vec4 o_color;

void main() {
    o_color = texture(u_texture, v_texCoords);
}
