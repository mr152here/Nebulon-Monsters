#version 330 core
layout (location = 0) in vec2 position;    //per vertex
layout (location = 1) in vec3 color;       //per instance
layout (location = 2) in vec4 m_pos_scale; //per instance

out VS_OUTPUT {
    vec4 color;
} OUT;

layout (std140) uniform Matrices {
    mat4 projection;
} m;

void main()
{
    mat4 model = mat4(
        m_pos_scale[2], 0.0, 0.0, 0.0,
        0.0, m_pos_scale[3], 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        m_pos_scale[0], m_pos_scale[1], 0.0, 1.0
    );

    gl_Position = m.projection * model * vec4(position, 0.0, 1.0);
    OUT.color = vec4(color, 1.0);
}
