const int MAX_GRADIENT_STOPS = 10;

attribute vec3 position;
uniform vec4 color;
uniform vec4 element_pos;
uniform vec2 tex_rate;
uniform int n_stops;
uniform vec4 color_stops[MAX_GRADIENT_STOPS];
uniform float stop_pos[MAX_GRADIENT_STOPS];
uniform vec2 gradient_pts[2];

varying vec4 aColor;
varying vec2 uv;

void main() {

    vec3 pos = vec3(element_pos.x + position.x*element_pos.z, element_pos.y + position.y*element_pos.w, position.z);

    uv = ((vec2(pos.x, pos.y) + 1.0)/2.0)*tex_rate;
    gl_Position = vec4(pos, 1.0);

    if (n_stops == 0) {
        aColor = color;
    } else {
        vec2 a = element_pos.xy + gradient_pts[0]*element_pos.zw;
        vec2 b = element_pos.xy + gradient_pts[1]*element_pos.zw;
        vec2 ab = b - a;
        float t = dot(pos.xy - a, ab)/dot(ab, ab);
        if (t <= stop_pos[0]) {
            aColor = color_stops[0];
        } else if (t >= stop_pos[n_stops - 1]) {
            aColor = color_stops[n_stops - 1];
        } else {
            for (int i = 1; i < MAX_GRADIENT_STOPS; i++) {
                if (t < stop_pos[i] || i+1 >= n_stops ) {
                    aColor = mix(color_stops[i-1], color_stops[i], (t - stop_pos[i-1])/(stop_pos[i] - stop_pos[i-1]));
                    break;
                }
            }
        }
    }
}