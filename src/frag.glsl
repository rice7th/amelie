#version 450
//in vec4 gl_FragCoord;
in vec4 color_frag;
in vec2 texcoord;
in flat int tex_id;
in vec4 frag_rounding;
in vec2 frag_pos;

uniform sampler2D tex[2];
uniform vec2 res;

layout(location = 0)out vec4 O_COLOR;

float rounded_rect_sdf(vec2 p, vec2 size, vec4 radius) {
    radius.xy = (p.x > 0.0) ? radius.xy : radius.zw;
    radius.x  = (p.y > 0.0) ? radius.x  : radius.y;
    vec2 q = abs(p) - size + radius.x;
    return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - radius.x;
}

float sdf(vec2 p) {
    return length(p) - 0.5;
}


void main() {
    vec4 fill;
    vec4 display_color;

    vec2 size = vec2(1)/2;

    fill = texture(tex[tex_id], texcoord);
    if (tex_id == -1) {
        fill = color_frag;
    }

    display_color = fill;

    float distance = rounded_rect_sdf(texcoord.xy - vec2(0.5), size, frag_rounding);
    float smooth_alpha = 1.0 - smoothstep(0.0, 2.0, sqrt(res.y*res.x)/4*distance); // Smooth alpha for antialiasing
    display_color.a = display_color.a * smooth_alpha;

    //O_COLOR = vec4(0.0);

    if (distance < 0) {
        //O_COLOR = mix(vec4(0.0f, 0.0f, 0.0f, 1.0f), display_color, smooth_alpha);
        O_COLOR = vec4(display_color.x, display_color.y, display_color.z, display_color.a);
    }
    //O_COLOR = vec4(1.0, 1.0, 1.0, display_color.a);
}

