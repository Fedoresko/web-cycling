precision mediump float;

attribute vec3 position;
uniform vec4 color;
uniform vec4 element_pos;
uniform vec2 tex_rate;

varying vec4 aColor;
varying vec2 uv;
varying vec2 pos2d;

void main() {
    vec3 pos = vec3(element_pos.x + position.x*element_pos.z, element_pos.y + position.y*element_pos.w, position.z);
    pos2d = pos.xy;

    uv = ((vec2(pos.x, pos.y) + 1.0)/2.0)*tex_rate;
    gl_Position = vec4(pos, 1.0);
    aColor = color;
}