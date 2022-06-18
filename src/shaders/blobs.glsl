uniform float f;
uniform vec2 r;

const int MAX_STEPS = 32;
const float EPS = 0.001;
const float END = 100.0;
const float START = 0.0;

vec2 opSU(vec2 d1, vec2 d2, float k) {
    float h = clamp(0.5 + 0.5 * (d2.x - d1.x) / k, 0.0, 1.0);
    float m = d1.y;
    if(d1.x > d2.x)
        m = d2.y;
    return vec2(mix(d2.x, d1.x, h) - k * h * (1.0 - h), m);
}

float sphere(vec3 p, float s) {
    return length(p) - s;
}

vec2 sdf(in vec3 p, float w) {
    float size = 1.;
    vec2 s1 = vec2(sphere((p - vec3(sin(f), sin(f * w) * 3. * w, -50.)), size), 1.);
    vec2 s2 = vec2(sphere((p - vec3(2. + sin(f), cos(-f) * 3., -50.)), size), 2.);
    vec2 s3 = vec2(sphere((p - vec3(-1.5 + sin(f - 0.5) * 3., -1.5, -50.)), size), 3.);
    vec2 s4 = vec2(sphere((p - vec3(-0.5, (sin(f - 0.5) * 2.), -50.)), size), 4.);
    vec2 s5 = vec2(sphere((p - vec3(cos(f) + 3.5, sin(f) * 2.5 - 2., 49.5)), size), 5.);
    vec2 s6 = vec2(sphere((p - vec3(2.5 + cos(f), sin(-f) * 5., -50.)), size), 6.);
    vec2 s7 = vec2(sphere((p - vec3(sin(f) / 2., cos(f) * 5., -49.)), size), 7.);
    vec2 r = opSU(s1, s2, 2.);
    r = opSU(r, s3, 2.);
    r = opSU(r, s4, 2.);
    r = opSU(r, s5, 2.);
    r = opSU(r, s6, 2.);
    r = opSU(r, s7, 2.);
    return r;
}

vec2 march(vec3 eye, vec3 dir, float s, float e, float w) {
    float d = s;
    for(int i = 0; i < MAX_STEPS; i++) {
        vec2 res = sdf(eye + d * dir, w);
        if(res.x < EPS)
            return vec2(d, res.y);
        d += res.x;
        if(d >= e)
            return vec2(e, 0.);
    }
    return vec2(e, 0.);
}

vec3 rayDir(float fov, vec2 uv) {
    vec2 xy = uv * 2. - 1.;
    xy.y = xy.y / (16. / 9.);
    float z = 2. / tan(radians(fov / 2.));
    return normalize(vec3(xy, -z));
}

void main() {
    vec2 uv = gl_FragCoord.xy / r;
    uv *= 0.5;
    uv.y += 0.25;
    uv.x += 0.25;
    vec3 eye = vec3(.0);
    vec3 dir = rayDir(60.0, uv);
    vec2 res = march(eye, dir, START, END, 0.);
    vec3 color = vec3(.0);

    if(res.x >= END - EPS) {
        gl_FragColor = vec4(vec3(0.), 1.0);
        return;
    }

    color = vec3(1., 1., .7);
    gl_FragColor = vec4(color, 1.0);
}