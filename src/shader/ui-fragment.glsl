precision mediump float;
const int MAX_GRADIENT_STOPS = 10;

varying vec4 aColor;
uniform vec2 iResolution;
uniform sampler2D iChannel0;
uniform bool blur;
uniform float opacity;
uniform int n_stops;
uniform vec4 color_stops[MAX_GRADIENT_STOPS];
uniform float stop_pos[MAX_GRADIENT_STOPS];
uniform vec2 gradient_pts[2];
varying vec2 uv;
varying vec2 pos2d;

//vec4 blur9(sampler2D image, vec2 uv, vec2 resolution, vec2 direction) {
//  vec4 color = vec4(0.0);
// vec2 off1 = vec2(1.3846153846) * direction;
//  vec2 off2 = vec2(3.2307692308) * direction;
// color += texture2D(image, uv) * 0.2270270270;
//  color += texture2D(image, uv + (off1 / resolution)) * 0.3162162162;
//  color += texture2D(image, uv - (off1 / resolution)) * 0.3162162162;
//  color += texture2D(image, uv + (off2 / resolution)) * 0.0702702703;
//  color += texture2D(image, uv - (off2 / resolution)) * 0.0702702703;
//  return color;
//}

vec3 rgb2hsv(vec3 c)
{
    vec4 K = vec4(0.0, -1.0 / 3.0, 2.0 / 3.0, -1.0);
    vec4 p = mix(vec4(c.bg, K.wz), vec4(c.gb, K.xy), step(c.b, c.g));
    vec4 q = mix(vec4(p.xyw, c.r), vec4(c.r, p.yzx), step(p.x, c.r));

    float d = q.x - min(q.w, q.y);
    float e = 1.0e-10;
    return vec3(abs(q.z + (q.w - q.y) / (6.0 * d + e)), d / (q.x + e), q.x);
}
vec3 hsv2rgb(vec3 c)
{
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec4 mainImage()
{
    const float Pi = 6.28318530718; // Pi*2

    // GAUSSIAN BLUR SETTINGS {{{
    const float Directions = 24.0; // BLUR DIRECTIONS (Default 16.0 - More is better but slower)
    const float Quality = 8.0; // BLUR QUALITY (Default 4.0 - More is better but slower)
    const float Size = 16.0; // BLUR SIZE (Radius)
    // GAUSSIAN BLUR SETTINGS }}}

    vec2 Radius = Size/iResolution.xy;

    // Normalized pixel coordinates (from 0 to 1)
    // Pixel colour
    vec4 Color = texture2D(iChannel0, uv);

    // Blur calculations
    for( float d=0.0; d<Pi; d+=Pi/Directions)
    {
		for(float i=1.0/Quality; i<=1.0; i+=1.0/Quality)
        {
			Color += texture2D( iChannel0, uv+vec2(cos(d),sin(d))*Radius*i);
        }
    }

    // Output to screen
    Color /= Quality * Directions - 15.0;
    return vec4(Color.xyz, 1.0);
}

vec4 applyGradient() {
    if (n_stops == 0) {
        return aColor;
    } else {
        vec2 a = gradient_pts[0];
        vec2 b = gradient_pts[1];
        vec2 ab = b - a;
        float t = dot(pos2d.xy - a, ab)/dot(ab, ab);
        if (t <= stop_pos[0]) {
            return color_stops[0];
        } else {
            for (int i = 1; i < MAX_GRADIENT_STOPS; i++) {
                if (i == n_stops) {
                    return color_stops[i - 1];
                } else if (t < stop_pos[i]) {
                    return mix(color_stops[i-1], color_stops[i], (t - stop_pos[i-1])/(stop_pos[i] - stop_pos[i-1]));
                }
            }
        }
    }
    return color_stops[0];
}


void main() {
  vec4 baseColor = applyGradient()*vec4(1.0,1.0,1.0,opacity);
  if (blur) {
    vec4 col = mainImage();
    gl_FragColor = vec4(mix(col.rgb, baseColor.rgb, baseColor.w), 1.0);
  } else {
    vec4 col = texture2D(iChannel0, uv);
    gl_FragColor = vec4(mix(col.rgb, baseColor.rgb, baseColor.w), 1.0);
  }
}

