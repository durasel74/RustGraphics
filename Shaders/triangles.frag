#version 330 core

in vec3 outColor;
out vec4 resultColor;

void main()
{
    resultColor = vec4(outColor, 1.0);
}
