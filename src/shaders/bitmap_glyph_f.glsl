#version 330 core

in VS_OUTPUT {
    flat vec3 color;
    vec2 tex_coord;
} IN;

out vec4 frag_color;

uniform sampler2D texture_font;

void main()
{
    frag_color = texture(texture_font, IN.tex_coord) * vec4(IN.color, 1.0);
}
