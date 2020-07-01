#version 140

in vec3 position;
in vec2 tex_coords;

out vec2 v_tex_coords;

uniform mat4 transform;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = transform * vec4(position, 1.0);
}
