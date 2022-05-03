precision mediump float;

varying vec2 texCoords;

uniform sampler2D texture;

uniform vec2 resolution;
uniform vec2 texrate;

float BSpline( float x )
{
	float f = x;
	if( f < 0.0 )
	{
		f = -f;
	}

	if( f >= 0.0 && f <= 1.0 )
	{
		return ( 2.0 / 3.0 ) + ( 0.5 ) * ( f* f * f ) - (f*f);
	}
	else if( f > 1.0 && f <= 2.0 )
	{
		return 1.0 / 6.0 * pow( ( 2.0 - f  ), 3.0 );
	}
	return 1.0;
}

float Cubic ( float x)
{
    const float B = 0.0;        // original bspline
    const float C = 0.5;
    float f = x;
    if( f < 0.0 )
    {
        f = -f;
    }
    if( f < 1.0 )
    {
        return ( ( 12.0 - 9.0 * B - 6.0 * C ) * ( pow(f,3.0) ) +
            ( -18.0 + 12.0 * B + 6.0 * C ) * ( pow(f,2.0) ) +
            ( 6.0 - 2.0 * B ) ) / 6.0;
    }
    else if( f >= 1.0 && f < 2.0 )
    {
        return ( ( -B - 6.0 * C ) * ( pow(f,3.0) )
            + ( 6.0 * B + 30.0 * C ) * ( pow(f,2.0) ) +
            ( - ( 12.0 * B ) - 48.0 * C  ) * f +
            8.0 * B + 24.0 * C)/ 6.0;
    }
    else
    {
        return 0.0;
    }
}

vec4 BiCubic( sampler2D textureSampler, vec2 TexCoord )
{
    float texelSizeX = 1.0 / resolution.x; //size of one texel
    float texelSizeY = 1.0 / resolution.y; //size of one texel
    vec4 nSum = vec4( 0.0, 0.0, 0.0, 0.0 );
    vec4 nDenom = vec4( 0.0, 0.0, 0.0, 0.0 );
    float a = fract( TexCoord.x * resolution.x ); // get the decimal part
    float b = fract( TexCoord.y * resolution.y ); // get the decimal part
    for( int m = -1; m <=2; m++ )
    {
        for( int n =-1; n<= 2; n++)
        {
			vec4 vecData = texture2D(textureSampler,
                               TexCoord + vec2(texelSizeX * float( m ),
					texelSizeY * float( n )));
			float f  = Cubic( float( m ) - a );
			vec4 vecCooef1 = vec4( f,f,f,f );
			float f1 = Cubic ( -( float( n ) - b ) );
			vec4 vecCoeef2 = vec4( f1, f1, f1, f1 );
            nSum = nSum + ( vecData * vecCoeef2 * vecCooef1  );
            nDenom = nDenom + (( vecCoeef2 * vecCooef1 ));
        }
    }
    return nSum / nDenom;
}

vec4 biLinearTex2D(sampler2D textureSampler, vec2 fragCoord)
{
    float texelSizeX = 1.0 / resolution.x; //size of one texel
    float texelSizeY = 1.0 / resolution.y; //size of one texel

    vec2 texel =  vec2(texelSizeX, texelSizeY);
	vec2 uv = fragCoord.xy;

    vec4 a = texture2D( textureSampler, uv + vec2(-0.5,-0.5)*texel );
    vec4 b = texture2D( textureSampler, uv + vec2(0.5,-0.5)*texel );
    vec4 c = texture2D( textureSampler, uv + vec2(0.5,0.5)*texel );
    vec4 d = texture2D( textureSampler, uv + vec2(-0.5,0.5)*texel );

    return mix(
        mix( a, b, 0.5),
        mix( c, d, 0.5), 0.5
    );
}

#define SHARPEN_FACTOR 8.0
#define THRESHOLD 0.1

vec4 sharpenMask (sampler2D tex, vec2 fragCoord)
{
    vec2 texel = texrate / resolution;

    // Sharpen detection matrix [0,1,0],[1,-4,1],[0,1,0]
    // Colors
    vec4 up = texture2D (tex, (fragCoord + vec2 (0, 1)*texel)*texrate);
    vec4 left = texture2D (tex, (fragCoord + vec2 (-1, 0)*texel)*texrate);
    vec4 center = texture2D (tex, fragCoord*texrate);
    vec4 right = texture2D (tex, (fragCoord + vec2 (1, 0)*texel)*texrate);
    vec4 down = texture2D (tex, (fragCoord + vec2 (0, -1)*texel)*texrate);


    vec4 variation = abs(1.0 - 4.0*center/(up + left + right + down));
    if ( variation.x > THRESHOLD || variation.y > THRESHOLD || variation.z > THRESHOLD || variation.w > THRESHOLD) {
        return center;
    }

    //vec4 splinevar = vec4(BSpline(variation.x),BSpline(variation.y),BSpline(variation.z),BSpline(variation.w));

    // Return edge detection
    return (1.0 + 4.0*SHARPEN_FACTOR)*center -SHARPEN_FACTOR*(up + left + right + down);
}

void main() {
//texture2D( texture, vec2(texCoords.s, texCoords.t) );
// BiCubic( texture, vec2(texCoords.s, texCoords.t) );
    gl_FragColor = texture2D( texture, vec2(texCoords.x, texCoords.y)*texrate );
}
