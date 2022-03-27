#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 FragPos;
in vec3 outNormal;
in vec3 outLightPos;
//in vec2 outTex;

out vec4 resultColor;

uniform Material material;
uniform Light light;

uniform int wire_mode;
//uniform sampler2D texture1;

void main()
{
    if (wire_mode == 0)
    {
        // ambient
        vec3 ambient = light.ambient * material.ambient;
        
        // diffuse 
        vec3 norm = normalize(outNormal);
        vec3 lightDir = normalize(outLightPos - FragPos);
        float diff = max(dot(norm, lightDir), 0.0);
        vec3 diffuse = light.diffuse * (diff * material.diffuse);

        // specular
        vec3 viewDir = normalize(-FragPos);
        vec3 reflectDir = reflect(-lightDir, norm);  
        float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
        vec3 specular = light.specular * (spec * material.specular);  

        vec3 result = ambient + diffuse + specular;
        resultColor = vec4(result, 1.0);
        
        //resultColor = vec4(outColor, 1.0f);
        // resultColor = texture(texture1, outTex);
    }
    else resultColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
