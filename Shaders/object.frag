#version 330 core

in vec3 outColor;
in vec2 outTex;
out vec4 resultColor;

uniform vec3 lightColor;
uniform float ambientStrength;
uniform int wire_mode;
uniform sampler2D texture1;

void main()
{
    if (wire_mode == 0)
    {
        vec3 ambient = ambientStrength * lightColor;
        resultColor = vec4(ambient * outColor, 1.0);
        
        //resultColor = vec4(outColor, 1.0f);
        // resultColor = texture(texture1, outTex);
    }
    else resultColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
