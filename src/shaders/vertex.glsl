#version 150

in vec4 position;
in vec2 tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    mat4 modelview = view * model;
    gl_Position = perspective * modelview * position;
}