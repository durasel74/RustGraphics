#version 330 core

// in VS_OUTPUT {
//    vec3 Color;
// } IN;

in vec3 outColor;
out vec4 Color;

void main()
{
    Color = vec4(outColor, 1.0f);
    // Color = vec4(IN.Color, 1.0f);
}