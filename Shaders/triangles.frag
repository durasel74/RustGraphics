#version 330 core

in vec3 outColor;
in vec2 outTex;
out vec4 resultColor;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    // resultColor = texture(texture2, outTex);
    // resultColor = texture(texture1, outTex) * texture(texture2, outTex);
    resultColor = mix(texture(texture1, outTex), texture(texture2, outTex), 0.3);
}
