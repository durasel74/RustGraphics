#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
    float dissolve;
};

struct DirLight {
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

struct PointLight {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    
    float constant;
    float linear;
    float quadratic;
};

struct SpotLight {
    vec3 position;
    vec3 direction;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

    float cutOff;
    float outerCutOff;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;
in mat4 View;

out vec4 FragColor;

#define DIR_LIGHT_COUNT 5
#define POINT_LIGHT_COUNT 30
#define SPOT_LIGHT_COUNT 2

uniform Material material;
uniform DirLight dirLights[DIR_LIGHT_COUNT];
uniform PointLight pointLights[POINT_LIGHT_COUNT];
uniform SpotLight spotLights[SPOT_LIGHT_COUNT];
uniform int dirLightCount;
uniform int pointLightCount;
uniform int spotLightCount;

uniform sampler2D texture_diffuse;
uniform sampler2D texture_specular;

uniform int draw_mode;

vec4 diff_tex = vec4(1.0f, 1.0f, 1.0f, 1.0f);
vec4 spec_tex = vec4(1.0f, 1.0f, 1.0f, 1.0f);

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir);
vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec3 CalcDirLightTest(DirLight light, vec3 normal, vec3 viewDir);
void PrepareTextureFragment();

void main()
{
    vec3 result = vec3(0.0f, 0.0f, 0.0f);
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(-FragPos);
    float dissolve = 1.0f;

    if (draw_mode == 0)
    {
        PrepareTextureFragment();
        if(diff_tex.a < 0.001) discard;
        dissolve = material.dissolve * diff_tex.w;

        for(int i = 0; i < dirLightCount; i++)
            result += CalcDirLight(dirLights[i], norm, viewDir);
        for(int i = 0; i < pointLightCount; i++)
            result += CalcPointLight(pointLights[i], norm, FragPos, viewDir);
        for(int i = 0; i < spotLightCount; i++)
            result += CalcSpotLight(spotLights[i], norm, FragPos, viewDir);
    }
    else if (draw_mode == 1)
    {
        DirLight dirLight = DirLight(
            vec3(0.4f, -0.5f, -0.4f),
            vec3(0.8f, 0.8f, 0.8f),
            vec3(1.0f, 1.0f, 1.0f),
            vec3(0.0f, 0.0f, 0.0f)
        );
        result = CalcDirLightTest(dirLight, norm, viewDir);
    }
    else result = vec3(1.0f, 1.0f, 1.0f);

    FragColor = vec4(result, dissolve);
}

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDirection = vec3(View * vec4(light.direction, 0.0));
    vec3 lightDir = normalize(-lightDirection);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    if (material.shininess == 0) spec = 0;

    vec3 ambient = light.ambient * material.ambient * vec3(diff_tex);
    vec3 diffuse = light.diffuse * diff * material.diffuse * vec3(diff_tex);
    vec3 specular = light.specular * spec * material.specular * vec3(spec_tex);
    return (ambient + diffuse + specular);
}

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightPosition = vec3(View * vec4(light.position, 1.0));
    vec3 lightDir = normalize(lightPosition - fragPos);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    if (material.shininess == 0) spec = 0;

    float distance = length(lightPosition - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + 
        light.quadratic * (distance * distance));
    
    vec3 ambient = light.ambient * material.ambient * vec3(diff_tex);
    vec3 diffuse = light.diffuse * diff * material.diffuse * vec3(diff_tex);
    vec3 specular = light.specular * spec * material.specular * vec3(spec_tex);
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
{
    vec3 lightPosition = vec3(View * vec4(light.position, 1.0));
    vec3 lightDirection = vec3(View * vec4(light.direction, 0.0));
    vec3 lightDir = normalize(lightPosition - fragPos);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    if (material.shininess == 0) spec = 0;

    float theta = dot(lightDir, normalize(-lightDirection));
    float epsilon = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

    vec3 ambient = light.ambient * material.ambient * vec3(diff_tex);
    vec3 diffuse = light.diffuse * diff * material.diffuse * vec3(diff_tex);
    vec3 specular = light.specular * spec * material.specular * vec3(spec_tex);
    diffuse *= intensity;
    specular *= intensity;
    return (ambient + diffuse + specular);
}

vec3 CalcDirLightTest(DirLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDirection = vec3(View * vec4(light.direction, 0.0));
    vec3 lightDir = normalize(-lightDirection);
    float diff = max(dot(normal, lightDir), 0.0);

    vec3 ambient = light.ambient * vec3(0.2f, 0.2f, 0.2f);
    vec3 diffuse = light.diffuse * diff * vec3(1.0f, 1.0f, 1.0f);
    return (ambient + diffuse);
}

void PrepareTextureFragment()
{
    diff_tex = texture(texture_diffuse, TexCoords);
    spec_tex = texture(texture_specular, TexCoords);
    if (vec3(diff_tex) == vec3(0.0f, 0.0f, 0.0f))
        diff_tex = vec4(1.0f, 1.0f, 1.0f, 1.0f);
    if (vec3(spec_tex) == vec3(0.0f, 0.0f, 0.0f))
        spec_tex = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
