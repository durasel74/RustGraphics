#version 330 core

in vec3 outColor;
in vec2 outTex;
out vec4 resultColor;

uniform int wire_mode;
uniform sampler2D texture1;

void main()
{
    if (wire_mode == 0)
    {
        resultColor = vec4(outColor, 1.0f);
        // resultColor = texture(texture2, outTex);
        // resultColor = texture(texture1, outTex) * texture(texture2, outTex);
        // resultColor = mix(texture(texture1, outTex), texture(texture2, outTex), 0.3);
    }
    else resultColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
