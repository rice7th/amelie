#version 450
layout(location = 0)in vec2 pos;
layout(location = 1)in vec3 col;
layout(location = 2)in vec2 uv;
layout(location = 3)in float tex_index;

//out vec4 gl_Position;

out vec3 color_frag;
out vec2 texcoord;
out float tex_id;

void main() {
    gl_Position = vec4(pos, 0, 1);
    color_frag = col;
    texcoord = uv;
    tex_id = tex_index;
}