#version 330 core

out vec4 FragColor;
in vec2 TexCoords;
uniform sampler2D texture_diffuse;

// const float offset = 1.0 / 300.0;
// const float offset = 1.0 / 800.0;
// vec2 offsets[9] = vec2[](
//     vec2(-offset,  offset), // top-left
//     vec2( 0.0f,    offset), // top-center
//     vec2( offset,  offset), // top-right
//     vec2(-offset,  0.0f),   // center-left
//     vec2( 0.0f,    0.0f),   // center-center
//     vec2( offset,  0.0f),   // center-right
//     vec2(-offset, -offset), // bottom-left
//     vec2( 0.0f,   -offset), // bottom-center
//     vec2( offset, -offset)  // bottom-right    
// );

void main()
{ 
    // Простой рендер буфера
    FragColor = texture(texture_diffuse, TexCoords);

    // // Инверсия
    // FragColor = vec4(vec3(1.0 - texture(texture_diffuse, TexCoords)), 1.0);

    // // Черно белое
    // FragColor = texture(texture_diffuse, TexCoords);
    // float average = 0.2126 * FragColor.r + 0.7152 * FragColor.g + 0.0722 * FragColor.b;
    // FragColor = vec4(average, average, average, 1.0);


    // Эффекты ядра

    // // Повышение резкости
    // float kernel[9] = float[](
    //     -2, -2, -2,
    //     -2,  17, -2,
    //     -2, -2, -2
    // );

    // // Размытие
    // float kernel[9] = float[](
    //     1.0 / 16, 2.0 / 16, 1.0 / 16,
    //     2.0 / 16, 4.0 / 16, 2.0 / 16,
    //     1.0 / 16, 2.0 / 16, 1.0 / 16  
    // );

    // // Выделение границ
    // float kernel[9] = float[](
    //     1, 1, 1,
    //     1, -8, 1,
    //     1, 1, 1
    // );
    
    // vec3 sampleTex[9];
    // for(int i = 0; i < 9; i++)
    // {
    //     sampleTex[i] = vec3(texture(texture_diffuse, TexCoords.st + offsets[i]));
    // }
    // vec3 col = vec3(0.0);
    // for(int i = 0; i < 9; i++)
    //     col += sampleTex[i] * kernel[i];
    
    // FragColor = vec4(col, 1.0);
}
