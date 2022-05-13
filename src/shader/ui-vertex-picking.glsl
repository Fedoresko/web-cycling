attribute vec3 position;
uniform int color;
uniform mat3 transform;
varying vec4 aColor;

void main() {
    aColor = vec4(mod(float(color / 65536) + 0.5, 256.0)/256.0, mod(float(color / 256) + 0.5, 256.0)/256.0, mod(float(color) + 0.5, 256.0)/256.0, 1.0);

    vec3 pos = position*transform; //vec3(element_pos.x + position.x*element_pos.z, element_pos.y + position.y*element_pos.w, position.z);

    gl_Position = vec4(pos, 1.0);
}