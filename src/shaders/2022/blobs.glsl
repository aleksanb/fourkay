uniform float f;

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
    // 1) We get [0, 1) coordinates for x,y by dividing by r.xy.
    // 2) Then we scale the y-coordinate so that 1 unit in x direction equals
    // the same number of real life cm in y direction.
    // 3) Then we translate the coordinate system so that 0 is in the middle of
    // the scene.
    // 4) Finally we increase the amount of scene shown to more than just 1 unit.
    vec2 uv = gl_FragCoord / 1920.;
    uv -= vec2(.5, 9./16./2.);
    uv *= 10;

    float I_angle = f/2.;
    vec2 uvt = vec2(uv.x * sin(I_angle) - uv.y * cos(I_angle), uv.x * cos(I_angle) + uv.y * sin(I_angle));
    float thump = max(0., sin(2.2253 * f)); // 3.1415 * f * 170. / 60. / 4. = 2.2253 * f
    if (f>35. && f < 58.)
    {
        uvt.x += uv.y * thump;
    }

    vec3 color = vec3(1., 1., .7);
    float m = mod(floor((uvt.x + uvt.y) / 2. * 3. + f), 3.);
    float l = mod(floor(uvt.x * 10. - f), 3.);

    if (f > 72.5)
    {
        f = 72.5;
    }

    // Pastel pulse
    if(f < 31.) {
        uv.y -= 0.5;
        uv /= 5.;
        uv *= min(abs(sin((f) / 5.)) * 3., 3.);
        
        float I_angle = f/2.;
        if(f>15.3) {
            uv = vec2(uv.x * sin(I_angle) - uv.y * cos(I_angle), uv.x * cos(I_angle) + uv.y * sin(I_angle));
        } 
        if(f>23.) {
            f += thump * 2.;
        }
        
        float c = sin((9.25 * uv.y) + (8.0 * f)) * cos((9.25 * uv.x));
        float shape = smoothstep(1.9 - clamp(distance(c + uv.y, 0.5) * 0.9, 0.0, 1.0), 1.0, 0.9);

        if(shape > 0.5) {
            color = vec3(1., 1., .7);
        } else {
            color = uv.y > 0.5 ? vec3(1., 0.41, 0.70) : vec3(0.52, 0.87, 0.83);
            color *= 1. - shape;
        }
    }

    // Bars
    if(f >= 32. && f < 44.) {
        float t = mod(floor(f - 32.), 12.);

        color = m == 0. && t < 3.?vec3(1., 1., 0.70):
                        vec3(0.7 + 0.3 * sin(m + t),
                        0.8 + 0.2 * cos(m + t + f / 4.),
                        0.8 + 0.2 * sin(m + t + f / 9.));

    }

    if(f >= 44.) {
        color = vec3(0.29, 0.0, 0.51);
        if(l == 2. && f >= 45.) {
            color = vec3(0.5, 0., 0.5);
        }
        if(l == 1. && f >= 46.) {
            color = vec3(0.1, 0.1, 0.44);
        }
    }

    if(f >= 36. && f < 56.) {
        float t = sdf(uv);
        if(t <= -1. - sin(f)) { // Set size of overlaid ball here
            color = vec3(1., 1., .7);
        }
    }

    if (f > 72.)
    {
        float ending_progression = max(0., 1. - (f - 72.) * 6.);
        uv = (uv - 0.5) * ending_progression + 0.5;
        uv.y -= 1.0 * (1. - ending_progression);
    }

    // Metaballs
    if(f >= 56.) {
        float angle = -f;

        if(f >= 63.2) {
            angle *= 2.;
            angle += thump;
            uv.x += sin(uv.y * thump * 4.); // Wobble the SDF
        }
        uv = vec2(uv.x * sin(angle) - uv.y * cos(angle), uv.x * cos(angle) + uv.y * sin(angle)); // rotate
        float t = sdf(uv);
        if(t <= 0.) {
            color = vec3(1., 1., .7);
            float t = mod(floor(f - 56.), 12.);
            color = vec3(0.8 + 0.2 * sin(m + t),
                    0.8 + 0.2 * cos(m + t + f / 4.),
                    0.8 + 0.2 * sin(m + t + f / 9.));
        }
    }

    gl_FragColor = vec4(color, 1.0);
}
