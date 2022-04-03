#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoord;

out vec3 FragPos;
out vec3 outNormal;
out vec4 outLightVector;
out vec2 outTex;

uniform vec4 lightVector;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat3 normalMatrix;

void main()
{
    gl_Position = projection * view * model * vec4(Position, 1.0);
    FragPos = vec3(view * model * vec4(Position, 1.0));
    outNormal = normalMatrix * Normal;
    outLightVector = view * lightVector;
    outTex = TexCoord;
}
