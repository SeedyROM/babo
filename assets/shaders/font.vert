#version 330 core 

layout (location = 0) in vec4 aPos;
// layout (location = 1) in vec2 textureCoordinate;

out vec2 textureCoordinates;

uniform mat4 transform;

void main() {
    textureCoordinates = aPos.zw;
    gl_Position = transform * vec4(aPos.xy, 0.0, 1.0);
}
