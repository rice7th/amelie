#version 450
//in vec4 gl_FragCoord;
in vec4 color_frag;
in vec2 texcoord;
in flat int tex_id;

uniform sampler2D tex[2];

layout(location = 0)out vec4 O_COLOR;

void main() {
    //gl_FragColor = vec4(tex_id,tex_id,tex_id, 1.0);
    O_COLOR = texture(tex[tex_id], texcoord);
    if (tex_id == -1) {
        O_COLOR = color_frag;
    }
    //O_COLOR = vec4(texcoord.x, texcoord.y, 0.0, 1.0);
}