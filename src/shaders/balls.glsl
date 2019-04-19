uniform float frame;
uniform vec2 resolution;

#define PI 3.14
#define R3 1.732051

vec2 rotate(vec2 uv, float radians) {
    return vec2(uv.x * cos(radians) - uv.y * sin(radians),
                uv.x * sin(radians) + uv.y * cos(radians));
}

vec2 kaleidoscope(vec2 uv, vec2 point, float add) {
    for(int i = 0; i < 14; i++) {
        uv = rotate(uv, PI * 2. / 14.);
        uv.x = abs(uv.x);
    }
    uv -= point;
    return uv;
}

float dist(vec2 uv) {
    return pow(pow(uv.x, 2.) + pow(uv.y, 2.),0.5);
}

vec3 color(float p, float offs) {
    
    return sin(vec3(12.23,45.23,56.2)+offs*3.)*.5+.5;
}


float dot_pattern(vec2 uv, float time, float crazynator) {
    uv = kaleidoscope(uv, vec2(0.3 + 0.2 * sin(time), 0.4), 2. + sin(time/3.));
    
    // Time varying pixel color
    float col = 0.;
    
    
    for (float i = 0.; i < 10.; i++) {
        for (float j = 0.; j < 10.; j++) {
            

            vec2 gv = vec2(i + mod(j, 2.) / 2. - 8., (j - 8.) * 0.707) / 16.;
            
            float size = sin(pow(i-8.,2.)+pow(j-8.,2.)+time+25.*dist(uv-gv)) * 0.05 * crazynator;
            if (dist(uv - gv) < size) {
                col += 1.;
            } else {
                col += 0.;
            }
        }
    }
    
    col -= 1.1;
    col = mod(col, 2.) - .95;
    
    return col;
}


float N(float p) {
    return fract(sin(p*123.34)*345.456);
}

vec4 mainImage(vec2 fragCoord, vec2 iResolution)
{
  vec4 fragColor;
    vec2 uv = (fragCoord/iResolution.xy) / vec2(1., 16./9.);
    uv -= vec2(.5, 9./16./2.);
    float iTime = frame / 60.;

    float duv= dot(uv, uv);
    
    float t = iTime / 3.;
    
    float intensity2 = 0.;

    float crazynator = max(1., min(8., (iTime - 12.)/2.));
    
    for(float i=0.; i<1.; i+=1./3.) {
        float t = fract(i+t);
        float z = mix(7., .1, t);
        float fade = smoothstep(0., .3, t)*smoothstep(1., .7, t);

        intensity2 += fade*t*dot_pattern(uv*z/1.8, iTime + i, crazynator);
    }
    
    
    vec3 colorized = color(iTime, dist(uv));
    float intensity1 = dot_pattern(uv, iTime, 1.);
    
    float time_stepper = iTime - 6.;
    float output_intensity = smoothstep(1., 0., time_stepper) * intensity1 +
                             smoothstep(0., 1., time_stepper) * intensity2;
    
    float colonator = max(0., min(1., (iTime - 18.)/2.));
    vec3 fadeout_colorized = colorized * (1. - colonator) + vec3(133./255., 243./255., 159./255.) * colonator;
    vec3 output_ready_for_fade = output_intensity*colorized;
    vec3 faded_output = output_ready_for_fade * (1. - colonator) + vec3(133./255., 243./255., 159./255.) * colonator;
    // Output to screen
    fragColor = vec4(faded_output ,1.0);

    return fragColor;
}

void main()
{
  vec2 position = gl_FragCoord.xy / resolution;

  gl_FragColor = mainImage(gl_FragCoord.xy, resolution);
}
