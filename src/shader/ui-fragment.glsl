precision mediump float;

varying vec4 aColor;

uniform vec2 iResolution;
uniform sampler2D iChannel0;
//uniform vec2 direction;
uniform bool blur;
varying vec2 uv;

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
    // vec2 uv = fragCoord/iResolution.xy;
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


void main() {
  if (blur) {
    vec4 col = mainImage();
    //vec3 hsl = rgb2hsv(col.rgb);
    //hsl.z = hsl.z/2.0;
    //vec3 rgb = hsv2rgb(hsl);

    gl_FragColor = vec4(mix(col.rgb, aColor.rgb, aColor.w), 1.0);
  } else {
    vec4 col = texture2D(iChannel0, uv);
    gl_FragColor = vec4(mix(col.rgb, aColor.rgb, aColor.w), 1.0);;
  }
}
