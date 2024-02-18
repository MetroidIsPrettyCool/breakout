#version 330 core
uniform vec3 offset;

in vec3 position;
in vec3 color;

out vec3 f_color;

void main() {
    gl_Position = vec4(position.xyz + offset.xyz, 1.0);
    f_color = color;
}
