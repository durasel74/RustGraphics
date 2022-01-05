#version 330 core

out vec4 Color;

uniform vec4 ourColor;

void main()
{
    Color = ourColor;
}