#version 450
//in vec4 gl_FragCoord;
in vec3 color_frag;
in vec2 texcoord;
in float tex_id;

uniform sampler2D tex[2];

layout(location = 0)out vec4 O_COLOR;

void main() {
    //gl_FragColor = vec4(tex_id,tex_id,tex_id, 1.0);
    int index = int(tex_id);
    O_COLOR = texture(tex[index], texcoord);
}