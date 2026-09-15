#version 330 core

in VS_OUTPUT {
    vec2 tex_coord;
} IN;

out vec4 frag_color;

uniform vec3 color;
uniform sampler2D texture_sprites;

void main()
{
    frag_color = texture(texture_sprites, IN.tex_coord) * vec4(color, 1.0);
}
