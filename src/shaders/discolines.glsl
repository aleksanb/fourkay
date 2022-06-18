uniform float f;
uniform vec2 r;

void main() {
    vec2 uv = gl_FragCoord.xy / r;
    uv.y -= .3;
    uv *= 2.;
    vec3 col = vec3(abs(cos(f)), abs(sin(f)), 0.3);
    float c = sin((9.25 * uv.y) + (8.0 * f)) * cos((9.25 * uv.x));
    float shape = smoothstep(1.0 - clamp(distance(c + uv.y, 0.5) * 0.5, 0.0, 1.0), 1.0, 0.9);
    col *= (1.0 - shape);
    gl_FragColor = vec4(col, 1.0);
}