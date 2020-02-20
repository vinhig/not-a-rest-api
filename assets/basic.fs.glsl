#version 330

out vec4 fragColour;

uniform sampler2D diffuseTexture;

in vec2 oUV;

void main() {
    vec4 diffuseColor = texture2D(diffuseTexture, oUV).rgba;
    fragColour = diffuseColor;
}

