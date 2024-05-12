#version 330
//in vec4 gl_FragCoord;
in vec3 color_frag;
in vec2 texcoord;

void main() {
    gl_FragColor = vec4(color_frag, 1.0);
    // gl_FragColor = vec4(texcoord.x, texcoord.y, 0.0, 1.0); // Texture instead
}