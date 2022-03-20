#version 330 core

in vec3 FragPos;
in vec3 outNormal;
in vec3 outLightPos;
//in vec2 outTex;

out vec4 resultColor;

uniform vec3 objectColor;
uniform vec3 lightColor;

uniform float ambientStrength;
uniform float specularStrength;
uniform int wire_mode;
//uniform sampler2D texture1;

void main()
{
    if (wire_mode == 0)
    {
        float ambientStrength = 0.3;
        vec3 ambient = ambientStrength * lightColor;

        vec3 norm = normalize(outNormal);
        vec3 lightDir = normalize(outLightPos - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = diff * lightColor;

        float specularStrength = 1.0;
        vec3 viewDir = normalize(-FragPos);
        vec3 reflectDir = reflect(-lightDir, norm);
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), 128);
        vec3 specular = specularStrength * spec * lightColor;  

        vec3 result = (ambient + diffuse + specular) * objectColor;
        resultColor = vec4(result, 1.0);
        
        //resultColor = vec4(outColor, 1.0f);
        // resultColor = texture(texture1, outTex);
    }
    else resultColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
