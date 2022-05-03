attribute vec3 position;
uniform vec4 color;
uniform vec4 element_pos;
uniform vec2 tex_rate;
varying vec4 aColor;
varying vec2 uv;

void main() {
    aColor = color;

    vec3 pos = vec3(element_pos.x + position.x*element_pos.z, element_pos.y + position.y*element_pos.w, position.z);

    gl_Position = vec4(pos, 1.0);
    uv = ((vec2(pos.x, pos.y) + 1.0)/2.0)*tex_rate;
}