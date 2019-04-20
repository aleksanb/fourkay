uniform float f;
uniform vec2 r;
// ray marching
const float max_iterations = 55.;
const float stop_threshold = 0.001;
const float grad_step = .1;
const float clip_far = 250.0;
const float p = 2.;
const float q = 5.;

// math
const float PI = 3.14159265359;
const float DEG_TO_RAD = PI / 180.0;
float steps;

float smin( float a, float b, float k )
{
    float res = exp2( -k*a ) + exp2( -k*b );
    return -log2( res )/k;
}

// distance function
float dist_sphere( vec3 pos, float r ) {
    return length( pos ) - r;
}

float dist_box( vec3 pos, vec3 size ) {
    return length( max( abs( pos ) - size, 0.0 ) );
}

vec3 torus_knot_pos(float time) {
    float r = 6. + cos(time * q);
    return vec3(r * cos(p * time),
                r * sin(p * time),
                -sin(q * time));
}

vec3 torus_knot_pos_x(float time) {
    float r = 6. + cos(time * q);
    return vec3(-sin(q * time),
                r * cos(p * time),
                r * sin(p * time));
}

// get distance in the world
float dist_field( vec3 pos, float time) {
    // ...add objects here...
    
    vec3 pos01 = torus_knot_pos(mod(time, 2.*PI));
    // object 0 : sphere
    float d0 = dist_sphere( pos  + pos01, 2. );
    // object 1 : cube
    float d1 = dist_box( pos  + pos01, vec3( 1.4 + 0.3*sin(time) ) );
    float final_value = max( d0, -d1 );
    for (float i=0.; i<6.; i++) {
        vec3 pos23 = torus_knot_pos(mod(time+i/4., 2.*PI));
        // object 0 : sphere
        float d2 = dist_sphere( pos  + pos23, 2. );
        // object 1 : cube
        float d3 = dist_box( pos  + pos23, vec3( 1.4 + 0.3*sin(time) ) );
        final_value = smin(final_value, max(d2, -d3), 6.4);
    }
    
    pos.x += 4.;
    pos.y -= 4.;
    
    vec3 pos01_x = torus_knot_pos_x(mod(time-PI/2., 2.*PI));
    // object 0 : sphere
    float d0_x = dist_sphere( pos  + pos01_x, 2. );
    // object 1 : cube
    float d1_x = dist_box( pos  + pos01_x, vec3( 1.4 + 0.3*sin(time) ) );
    float final_value_x = max( d0_x, -d1_x );
    for (float i=0.; i<6.; i++) {
        vec3 pos23_x = torus_knot_pos_x(mod(time+i/4.-PI/2., 2.*PI));
        // object 0 : sphere
        float d2_x = dist_sphere( pos  + pos23_x, 2. );
        // object 1 : cube
        float d3_x = dist_box( pos  + pos23_x, vec3( 1.4 + 0.3*sin(time) ) );
        final_value_x = smin(final_value_x, max(d2_x, -d3_x), 6.4);
    }
    
    /*vec3 pos45 = torus_knot_pos(mod(time+2., 2.*PI));
    // object 0 : sphere
    float d4 = dist_sphere( pos  + pos45, 2. );
    // object 1 : cube
    float d5 = dist_box( pos  + pos45, vec3( 1.4 + 0.3*sin(time) ) );*/
        
    // union     : min( d0,  d1 )
    // intersect : max( d0,  d1 )
    // subtract  : max( d1, -d0 )
    return smin(final_value, final_value_x, 4.);
    //return max(d2, -d3);
}

// phong shading
vec3 shading( vec3 v, vec3 n, vec3 eye ) {
    // ...add lights here...
    
    float shininess = 16.0;
    
    vec3 final = vec3( 0.0 );
    
    vec3 ev = normalize( v - eye );
    vec3 ref_ev = reflect( ev, n );
    
    // light 0
    {
        vec3 light_pos   = vec3( 20.0, 20.0, 20.0 );
        vec3 light_color = vec3( 1.0, 0.7, 0.7 );
    
        vec3 vl = normalize( light_pos - v );
    
        float diffuse  = max( 0.0, dot( vl, n ) );
        float specular = max( 0.0, dot( vl, ref_ev ) );
        specular = pow( specular, shininess );
        
        final += light_color * ( diffuse + specular ); 
    }
    
    // light 1
    {
        vec3 light_pos   = vec3( -20.0, -20.0, -20.0 );
        vec3 light_color = vec3( 0.3, 0.7, 1.0 );
    
        vec3 vl = normalize( light_pos - v );
    
        float diffuse  = max( 0.0, dot( vl, n ) );
        float specular = max( 0.0, dot( vl, ref_ev ) );
        specular = pow( specular, shininess );
        
        final += light_color * ( diffuse + specular ); 
    }

    return final;
}

// get gradient in the world
vec3 gradient( vec3 pos, float time ) {
    const vec3 dx = vec3( grad_step, 0.0, 0.0 );
    const vec3 dy = vec3( 0.0, grad_step, 0.0 );
    const vec3 dz = vec3( 0.0, 0.0, grad_step );
    return normalize (
        vec3(
            dist_field( pos + dx, time ) - dist_field( pos - dx, time ),
            dist_field( pos + dy, time ) - dist_field( pos - dy, time ),
            dist_field( pos + dz, time ) - dist_field( pos - dz, time )         
        )
    );
}

// ray marching
float ray_marching( vec3 origin, vec3 dir, float start, float end, float time) {
    float depth = start;
    for ( float i = 0.; i < max_iterations; i++ ) {
        steps = i * 1.;
        float dist = dist_field( origin + dir * depth, time );
        if ( dist < stop_threshold ) {
            return depth;
        }
        depth += dist;
        if ( depth >= end) {
            return end;
        }
    }
    return end;
}

// get ray direction
vec3 ray_dir( float fov, vec2 size, vec2 pos ) {
    vec2 xy = pos - size * 0.5;

    float cot_half_fov = tan( ( 90.0 - fov * 0.5 ) * DEG_TO_RAD );  
    float z = size.y * 0.5 * cot_half_fov;
    
    return normalize( vec3( xy, -z ) );
}

// camera rotation : pitch, yaw
mat3 rotationXY( vec2 angle ) {
    vec2 c = cos( angle );
    vec2 s = sin( angle );
    
    return mat3(
        c.y      ,  0.0, -s.y,
        s.y * s.x,  c.x,  c.y * s.x,
        s.y * c.x, -s.x,  c.y * c.x
    );
}


vec4 mainImage(vec2 fragCoord, vec2 iResolution)
{
  vec4 fragColor;

    float iTime = f / 60.;
        // Background
    vec2 uv = fragCoord/iResolution.xy;
    float intensity = min(1., max(0.,1. * sin(uv.y * 100.) + (2. + sin(iTime * 4.) * 2.) * sin(uv.x * 100.)));
    
    // default ray dir
    vec3 dir = ray_dir( 45.0, iResolution.xy, fragCoord.xy );
    
    // default ray origin
    vec3 eye = vec3( 0.0, 0.0, 15.0 );

    // rotate camera
    mat3 rot = rotationXY( vec2( iTime * 0.1) );
    dir = rot * dir;
    eye = rot * eye;
    
    // ray marching
    float depth = ray_marching( eye, dir, 0.0, clip_far, iTime);
    if ( depth >= clip_far ) {
        float glow = min(max(0., (steps - 15.) * 0.08), 1.);
        fragColor = vec4( vec3(glow) + vec3(0.8, 0.8, 0.3) * intensity, 1.0 );
        return fragColor;
    }
    
    // shading
    vec3 pos = eye + dir * depth;
    vec3 n = gradient( pos, iTime );
    fragColor = vec4( shading( pos, n, eye ), 1.0 );

    return fragColor;
}

void main()
{
  vec2 position = gl_FragCoord.xy / r;

  gl_FragColor = mainImage(gl_FragCoord.xy, r);
}
