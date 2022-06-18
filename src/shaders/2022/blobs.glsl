uniform float f;
uniform vec2 r;

float opSU(float d1, float d2, float k) {
    float h = clamp(0.5 + 0.5 * (d2 - d1) / k, 0.0, 1.0);
    return mix(d2, d1, h) - k * h * (1.0 - h);
}

float sphere(vec2 p, float s) {
    return length(p) - s;
}

float sdf(in vec2 p) {
    float size = 0.5;
    float s1 = sphere((p - vec2(sin(f), 2. + sin(f) * .8)), size);
    float s2 = sphere((p - vec2(2. + sin(f), 1.4 + cos(-f) * 1.2)), size);
    float s3 = sphere((p - vec2(1.5 + cos(f), -.1)), size);
    float s4 = sphere((p - vec2(3.5, 2. + sin(f) * .9)), size);
    float s5 = sphere((p - vec2(5. + sin(f), cos(f) + 2.5)), size);
    float s6 = sphere((p - vec2(4.5 + cos(f), sin(-f) * .5)), size);
    float s7 = sphere((p - vec2(3.5 + sin(f) / 2., cos(f) * .9)), size);
    float r = opSU(s1, s2, 2.);
    r = opSU(r, s3, 2.);
    r = opSU(r, s4, 2.);
    r = opSU(r, s5, 2.);
    r = opSU(r, s6, 2.);
    r = opSU(r, s7, 2.);

    float s8 = sphere((p - vec2(2.5)), 19. - f * 0.28);
    r = opSU(r, s8, 2.);

    return r;
}

void main() {

    vec2 uv = (gl_FragCoord / r.xy) / vec2(1., 16. / 9.);
    vec3 color = vec3(1., 0.41, 0.70);

    if(f >= 46.57) {
        float m = mod(floor((uv.x + uv.y) / 2. * 3. + f), 3.);
        if(m == 0.) {
            color = vec3(1., 0.75, 0.79);
        } else if(m == 1.) {
            color = vec3(0.78, 0.08, 0.52);
        }

        float t = sdf(uv);
        if(t <= 0.) {
            color = vec3(1., 1., .7);
        }

    }

    if(f < 46.57) {
        uv.y -= .35;
        uv /= 5.;
        uv *= min(abs(sin((f) / 15.)) * 3., 7.);
        color = vec3(1., 0.41, 0.70);

        float c = sin((9.25 * uv.y) + (8.0 * f)) * cos((9.25 * uv.x));
        float shape = smoothstep(1.9 - clamp(distance(c + uv.y, 0.5) * 0.9, 0.0, 1.0), 1.0, 0.9);

        if(shape > 0.5) {
            color = vec3(1., 1., .7);
        } else {
            color = uv.y > 0.5 ? vec3(1., 0.41, 0.70) : vec3(0.52, 0.87, 0.83);
            color *= 1. - shape;
        }

    }

    gl_FragColor = vec4(color, 1.0);
}