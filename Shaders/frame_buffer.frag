#version 330 core

out vec4 FragColor;
in vec2 TexCoords;
uniform sampler2D texture_diffuse;

void main()
{ 
    FragColor = texture(texture_diffuse, TexCoords);
    // FragColor = vec4(vec3(1.0 - texture(texture_diffuse, TexCoords)), 1.0);
}
