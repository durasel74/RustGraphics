#version 330 core

struct Material {
    sampler2D diffuse;
    sampler2D specular;
    float shininess;
};

struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float constant;
    float linear;
    float quadratic;

    float cutOff;
    float outerCutOff;
};

in vec3 FragPos;
in vec3 outNormal;
in vec4 outLightPos;
in vec3 outLightDirection;
in vec2 outTex;

out vec4 resultColor;

uniform Material material;
uniform Light light;
uniform int wire_mode;

void main()
{
    if (wire_mode == 0)
    {
        // // ambient
        // vec3 ambient = light.ambient * vec3(texture(material.diffuse, outTex));
        // // vec3 ambient = light.ambient * material.ambient;
        
        // // diffuse
        // vec3 norm = normalize(outNormal);
        // float posMode = outLightPos.w;
        // vec3 lightDir = normalize(-outLightDirection);
        // if (posMode == 1)
        // {
        //     lightDir = normalize(vec3(outLightPos) - FragPos);
        // }
        // float diff = max(dot(norm, lightDir), 0.0);
        // vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, outTex));
        // // vec3 diffuse = light.diffuse * (diff * material.diffuse);

        // // specular
        // vec3 viewDir = normalize(-FragPos);
        // vec3 reflectDir = reflect(-lightDir, norm);
        // float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        // vec3 specular = light.specular * spec * vec3(texture(material.specular, outTex));
        // //vec3 specular = light.specular * (spec * 0.6);

        // if (posMode == 1)
        // {
        //     float distance = length(vec3(outLightPos) - FragPos);
        //     float attenuation = 1.0 / (light.constant + light.linear * distance + 
        //         light.quadratic * (distance * distance));
        //     ambient *= attenuation;
        //     diffuse *= attenuation;
        //     specular *= attenuation;
        // }
        // resultColor = vec4(ambient + diffuse + specular, 1.0);

        
        

        // ambient
        vec3 ambient = light.ambient * vec3(texture(material.diffuse, outTex));
        // vec3 ambient = light.ambient * material.ambient;

        vec3 norm = normalize(outNormal);
        vec3 lightDir = normalize(vec3(outLightPos) - FragPos);
        
        float theta = dot(lightDir, normalize(-outLightDirection));
        float epsilon = light.cutOff - light.outerCutOff;
        float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

        // diffuse
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = light.diffuse * diff * vec3(texture(material.diffuse, outTex));

        // specular
        vec3 viewDir = normalize(-FragPos);
        vec3 reflectDir = reflect(-lightDir, norm);
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        vec3 specular = light.specular * spec * vec3(texture(material.specular, outTex));

        diffuse  *= intensity;
        specular *= intensity;

        resultColor = vec4(ambient + diffuse + specular, 1.0);
    }
    else resultColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
