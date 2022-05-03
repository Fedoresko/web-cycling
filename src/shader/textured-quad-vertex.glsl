attribute vec4 vertexData; // <vec2 position, vec2 texCoords>
uniform float depth;
varying vec2 texCoords;

void main() {
    gl_Position = vec4(vertexData.xy, depth, 1.0);

    texCoords = vertexData.zw;
}
