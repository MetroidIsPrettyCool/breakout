#version 330 core
uniform float window_aspect;

in vec3 position;
in vec3 color;

out vec3 f_color;

void main() {
    gl_Position = vec4(position.xyz, 1.0);

    if (window_aspect > 1.0) {
        gl_Position.x /= window_aspect;
    } else {
        gl_Position.y *= window_aspect;
    }

    f_color = color;
}
