#version 130
uniform float frame;

void main() {
    gl_FragColor = vec4(gl_FragCoord.x / 1024.0, cos(frame / 10.0) / 2.0 + 0.5, gl_FragCoord.y / 768.0, 1.0);
}
