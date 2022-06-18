uniform float f;
uniform vec2 r;

const int MAX_STEPS = 32;
const float EPS = 0.001;
const float END = 100.0;
const float START = 0.0;

float opSU(float d1, float d2, float k) {
    float h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
    return mix(d2, d1, h) - k * h * (1.0 - h);
}

float sphere(vec3 p, float s) {
    return length(p) - s;
}

float sdf(in vec3 p) {
    float s1 = sphere((p - vec3(sin(f), 2. + sin(f) * 3., -50.)), 1.);
    float s2 = sphere((p - vec3(2. + sin(f), cos(-f) * 3., -50.)), 1.);
    float s3 = sphere((p - vec3(-5.5 + sin(f - 0.5) * 3., -2.5, -50.)), 1.);
    float s4 = sphere((p - vec3(-2.5, (sin(f - 0.5) * 2.), -50.)), 1.);
    float s5 = sphere((p - vec3(cos(f) + 3.5, sin(f) * 2.5 - 2., 49.5)), 1.);
    float s6 = sphere((p - vec3(4.5 + cos(f), sin(-f) * 5., -50.)), 1.);
    float s7 = sphere((p - vec3(sin(f) / 2., cos(f) * 5., -49.)), 1.);
    float r = opSU(s1, s2, 2.);
    r = opSU(r, s3, 2.);
    r = opSU(r, s4, 2.);
    r = opSU(r, s5, 2.);
    r = opSU(r, s6, 2.);
    r = opSU(r, s7, 2.);
    return r;
}

float march(vec3 eye, vec3 dir, float s, float e) {
    float d = s;
    for(int i = 0; i < MAX_STEPS; i++) {
        float res = sdf(eye + d * dir);
        if(res < EPS)
            return d;
        d += res;
        if(d >= e)
            return e;
    }
    return e;
}

vec3 rayDir(float fov, vec2 uv) {
    vec2 xy = uv * 2. - 1.;
    xy.y = xy.y / (16. / 9.);
    float z = 2. / tan(radians(fov / 2.));
    return normalize(vec3(xy, -z));
}

void main() {
    vec2 uv = gl_FragCoord.xy / r;
    vec3 eye = vec3(.0);
    vec3 dir = rayDir(60.0, uv);
    float res = march(eye, dir, START, END);
    vec3 color = vec3(255. / 255., 105. / 255., 180. / 255.);

    if(res >= END - EPS) {
        float m = mod(floor(uv.x * 29. + f), 3.);
        if(m == 0.) {
            color = vec3(255. / 255., 192. / 255., 203. / 255.);
        } else if(m == 1.) {
            color = vec3(199. / 255., 21. / 255., 133. / 255.);
        }
        gl_FragColor = vec4(color, 1.0);
        return;
    }

    color = vec3(1., 1., .7);
    gl_FragColor = vec4(color, 1.0);
}