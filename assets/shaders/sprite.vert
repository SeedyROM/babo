#version 330 core 

layout (location = 0) in vec2 vertexPosition;
layout (location = 1) in vec2 textureCoordinate;

out vec2 textureCoordinates;

uniform mat4 transform;

void main() {
    textureCoordinates = textureCoordinate;
    gl_Position = transform * vec4(vertexPosition, 0.0, 1.0);
}
