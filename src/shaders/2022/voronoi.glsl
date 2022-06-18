uniform float f;
uniform vec2 r;

vec2 hash(vec2 p) {
    p = vec2(dot(p, vec2(127.1, 311.7)), dot(p, vec2(269.5, 183.3)));
    return fract(sin(p) * 18.5453);
}

vec2 voronoi(in vec2 x) {
    vec2 n = floor(x);
    vec2 a = fract(x);
    vec3 m = vec3(8.0);
    for(int j = -1; j <= 1; j++) for(int i = -1; i <= 1; i++) {
            vec2 g = vec2(float(i), float(j));
            vec2 o = hash(n + g);
            vec2 r = g - a + (0.5 + 0.5 * sin(a + 6.2831 * o));
            float d = dot(r, r);
            if(d < m.x)
                m = vec3(d, o);
        }
    return vec2(sqrt(m.x), m.y + m.z);
}

void main() {
    vec2 uv = gl_FragCoord.xy / r;
    vec2 c = voronoi(20. * uv);
    vec3 col = 0.5 + 0.5 * abs(cos(f + c.y + vec3(1.0, 0.0, 0.4)));
    gl_FragColor = vec4(col, 1.0);
}