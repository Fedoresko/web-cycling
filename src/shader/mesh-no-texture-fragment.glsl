precision mediump float;

varying vec3 vNormal;
varying vec3 vWorldPos;

varying vec4 aColor;
varying vec3 fromFragmentToCamera;

varying vec4 worldPosition;

float shininess = 0.2;

vec3 sunlightColor = vec3(1.0, 1.0, 1.0);
vec3 sunlightDir = normalize(vec3(-1.0, 1.0, 0.5));

void main(void) {
//    if (dot(worldPosition, clipPlane) < 0.0) {
//        discard;
//    }

    vec3 ambient = vec3(0.24725, 0.1995, 0.0745) + vec3(0.3, 0.3, 0.3);

    vec3 normal = normalize(vNormal);
    float diff = max(dot(normal, -sunlightDir), 0.0);
    vec3 diffuse = diff * sunlightColor;

    vec3 reflectDir = reflect(-sunlightDir, normal);
    float spec = pow(max(dot(normalize(fromFragmentToCamera), reflectDir), 0.0), 32.0);
    vec3 specular = shininess * spec * vec3(0.628281, 0.555802, 0.366065);

    vec4 lighting = vec4(ambient + diffuse + specular, 1.0);
    
    gl_FragColor = aColor * lighting;
}
