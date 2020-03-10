attribute vec2 a_position;
attribute vec2 a_texCoords;
uniform mat3 u_perspective;
varying vec2 v_texCoords;

void main() {
     v_texCoords = a_texCoords;
     gl_Position = vec4(u_perspective * vec3(a_position, 1.0), 1.0);
}
