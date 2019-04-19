#version 130
uniform float frame;
uniform vec2 resolution;

void main() {
    vec2 position = gl_FragCoord.xy / resolution;
    gl_FragColor = vec4(position.x, cos(frame / 10.0) / 2.0 + 0.5, position.y, 1.0);
}
