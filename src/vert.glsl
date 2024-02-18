#version 330 core
uniform vec3 offset;
uniform vec2 window_aspect;

in vec3 position;
in vec3 color;

out vec3 f_color;

void main() {
    gl_Position = vec4(position.xyz + offset.xyz, 1.0);

    if (window_aspect.x > window_aspect.y) {
        gl_Position.x *= window_aspect.y / window_aspect.x;
    } else {
        gl_Position.y *= window_aspect.x / window_aspect.y;
    }

    f_color = color;
}
