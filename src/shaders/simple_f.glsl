#version 330 core

in VS_OUTPUT {
    vec4 color;
} IN;

out vec4 frag_color;

void main()
{
    frag_color = IN.color;
}
