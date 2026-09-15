#version 330 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 tex_coord;

out VS_OUTPUT {
    vec2 tex_coord;
} OUT;

uniform mat4 model;

layout (std140) uniform Matrices {
    mat4 projection;
} m;

void main()
{
    gl_Position = m.projection * model * vec4(position, 0.0, 1.0);
    OUT.tex_coord = tex_coord;
}
