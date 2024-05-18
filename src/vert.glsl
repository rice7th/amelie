#version 450
layout(location = 0)in vec2 pos;
layout(location = 1)in vec3 col;
layout(location = 2)in vec2 uv;
layout(location = 3)in int tex_index;
layout(location = 4)in float rot;

//out vec4 gl_Position;

out vec3 color_frag;
out vec2 texcoord;
out int tex_id;

void main() {
    vec2 position = vec2(
        pos.x * cos(rot) - pos.y * sin(rot),
        pos.x * sin(rot) + pos.y * cos(rot));
    gl_Position = vec4(position, 0, 1);
    color_frag = col;
    texcoord = uv;
    tex_id = tex_index;
}