attribute vec3 position;
attribute vec3 normal;
attribute vec3 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 perspective;
uniform mat4 normalMat;

varying vec3 vNormal;
varying vec3 vWorldPos;
varying vec4 worldPosition;
varying vec4 aColor;

uniform vec3 cameraPos;
varying vec3 fromFragmentToCamera;

void main (void) {
  worldPosition = model * vec4(position, 1.0);

  gl_Position = perspective * view * worldPosition;

  vNormal = vec3(normalMat * vec4(normal, 0.0));
  vWorldPos = worldPosition.xyz;
  fromFragmentToCamera = cameraPos - worldPosition.xyz;

  aColor = vec4(color, 1.0);
}
