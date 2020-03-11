#version 140

in vec2 in_position;
in vec2 in_texture_coords;

uniform mat3 u_perspective;

out vec2 texture_coords;

void main() {
    texture_coords = in_texture_coords;
    gl_Position = vec4(u_perspective * vec3(in_position, 1.0), 1.0);
}
