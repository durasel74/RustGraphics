#version 330 core

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
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

vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir);
vec4 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec4 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir);
vec4 CalcDirLightTest(DirLight light, vec3 normal, vec3 viewDir);
void PrepareTextureFragment();

void main()
{
    vec4 result = vec4(0.0f, 0.0f, 0.0f, 1.0f);
    vec3 norm = normalize(Normal);
    vec3 viewDir = normalize(-FragPos);

    if (draw_mode == 0)
    {
        vec4 texColor = texture(texture_diffuse, TexCoords);
        if(texColor.a < 0.001) discard;

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
    else result = vec4(1.0f, 1.0f, 1.0f, 1.0f);

    FragColor = result;
}

vec4 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDirection = vec3(View * vec4(light.direction, 0.0));
    vec3 lightDir = normalize(-lightDirection);
    float diff = max(dot(normal, lightDir), 0.0);
    vec3 reflectDir = reflect(-lightDir, normal);
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    if (material.shininess == 0) spec = 0;

    PrepareTextureFragment();

    vec4 ambient = vec4(light.ambient, 1.0f) * vec4(material.ambient, 1.0f) * diff_tex;
    vec4 diffuse = vec4(light.diffuse, 1.0f) * diff * vec4(material.diffuse, 1.0f) * diff_tex;
    vec4 specular = vec4(light.specular, 1.0f) * spec * vec4(material.specular, 1.0f) * spec_tex;
    return (ambient + diffuse + specular);
}

vec4 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
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

    PrepareTextureFragment();
    
    vec4 ambient = vec4(light.ambient, 1.0f) * vec4(material.ambient, 1.0f) * diff_tex;
    vec4 diffuse = vec4(light.diffuse, 1.0f) * diff * vec4(material.diffuse, 1.0f) * diff_tex;
    vec4 specular = vec4(light.specular, 1.0f) * spec * vec4(material.specular, 1.0f) * spec_tex;
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

vec4 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir)
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

    PrepareTextureFragment();

    vec4 ambient = vec4(light.ambient, 1.0f) * vec4(material.ambient, 1.0f) * diff_tex;
    vec4 diffuse = vec4(light.diffuse, 1.0f) * diff * vec4(material.diffuse, 1.0f) * diff_tex;
    vec4 specular = vec4(light.specular, 1.0f) * spec * vec4(material.specular, 1.0f) * spec_tex;
    diffuse *= intensity;
    specular *= intensity;
    return (ambient + diffuse + specular);
}

vec4 CalcDirLightTest(DirLight light, vec3 normal, vec3 viewDir)
{
    vec3 lightDirection = vec3(View * vec4(light.direction, 0.0));
    vec3 lightDir = normalize(-lightDirection);
    float diff = max(dot(normal, lightDir), 0.0);

    vec3 ambient = light.ambient * vec3(0.2f, 0.2f, 0.2f);
    vec3 diffuse = light.diffuse * diff * vec3(1.0f, 1.0f, 1.0f);
    return vec4((ambient + diffuse), 1.0f);
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
