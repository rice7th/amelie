#version 450
layout(location = 0)in vec2 pos;
layout(location = 1)in vec4 col;
layout(location = 2)in vec2 uv;
layout(location = 3)in int tex_index;
layout(location = 4)in vec4 rounding;

//out vec4 gl_Position;

out vec4 color_frag;
out vec2 texcoord;
out int tex_id;
out vec4 frag_rounding;
out vec2 frag_pos;

void main() {
    //vec2 position = vec2(
    //    pos.x * cos(rot) - pos.y * sin(rot),
    //    pos.x * sin(rot) + pos.y * cos(rot));
    gl_Position = vec4(pos, 0, 1);
    color_frag = col;
    texcoord = uv;
    tex_id = tex_index;
    frag_rounding = rounding;
    frag_pos = pos;
}