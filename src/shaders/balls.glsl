uniform float frame;
uniform vec2 resolution;

float dist(vec2 uv) {
    return pow(pow(uv.x, 2.) + pow(uv.y, 2.),0.5);
}

vec4 mainImage(vec2 fragCoord, vec2 iResolution)
{
    vec2 uv = (fragCoord/iResolution.xy) / vec2(1., 16./9.);
    uv -= vec2(.5, 9./16./2.);
    float iTime = frame / 60.;
    

    // Time varying pixel color
    float col = 0.;
    
    
    for (float i = 0.; i < 16.; i++) {
        for (float j = 0.; j < 16.; j++) {

            vec2 gv = vec2(i + mod(j, 2.) / 2. - 8., (j - 8.) * 0.707) / 16.;
            
            float size = sin(pow(i-8.,2.)+pow(j-8.,2.)+iTime+25.*dist(uv-gv)) * 0.05;
            if (dist(uv - gv) < size) {
                col += 1.;
            } else {
                col += 0.;
            }
        }
    }
    
    col -= 0.1;
    col = mod(col, 2.) - .95;
    
    // Output to screen
    vec4 fragColor = vec4(vec3(col) ,1.0);

    return fragColor;
}

void main()
{
  vec2 position = gl_FragCoord.xy / resolution;

  gl_FragColor = mainImage(gl_FragCoord.xy, resolution);
}
