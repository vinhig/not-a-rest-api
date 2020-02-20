#version 330

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 textCoord;

out vec2 oUV;

void main() {
    gl_Position = vec4(position, 1.0);
    oUV = textCoord;
}

