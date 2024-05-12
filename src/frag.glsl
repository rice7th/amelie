#version 330
//in vec4 gl_FragCoord;
in vec3 color_frag;
in vec2 texcoord;

uniform sampler2D tex;

void main() {
    //gl_FragColor = vec4(color_frag, 1.0);
    gl_FragColor = texture(tex, texcoord);
}