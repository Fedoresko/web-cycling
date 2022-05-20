precision mediump float;

attribute vec3 position;
uniform vec4 color;
uniform vec2 tex_rate;
uniform mat3 transform;
uniform float opacity;

varying vec4 aColor;
varying vec2 uv;
varying vec2 pos2d;

void main() {
    pos2d = position.xy;
    vec3 pos = vec3(transform*vec3(pos2d, 1.0));
    pos.z = position.z;

    uv = ((vec2(pos.x, pos.y) + 1.0)/2.0)*tex_rate;
    gl_Position = vec4(pos, 1.0);
    aColor = vec4(color.r, color.g, color.b, color.a*opacity);
}