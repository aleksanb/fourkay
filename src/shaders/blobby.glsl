uniform float frame;
uniform sampler2D tDiffuse;

varying vec2 vUv;

const float MAX_STEPS = 32.;
const float EPS = .001;
const float END = 100.;
const float START = .0;
float steps;

vec2 uni(vec2 d1, vec2 d2) { 
  float m = d1.y;
  if (d1.x>d2.x) m = d2.y;
  return vec2(min(d1.x, d2.x), m);
}
vec2 suni(vec2 d1, vec2 d2, float k) { 
  float h = clamp(.5+.5*(d2.x-d1.x)/k, .0, 1.);
  float m = d1.y;
  if (d1.x>d2.x) m = d2.y;
  return vec2(mix(d2.x,d1.x,h)-k*h*(1.-h), m);
}
float sphere(vec3 p, float s) { 
  return length(p)-s;
}
float torus(vec3 p, vec2 t) {
  vec2 q = vec2(length(p.xy)-t.x,p.z);
  return length(q)-t.y;
}
float elli(in vec3 p, in vec3 r) {
  float k0 = length(p/r);
  float k1 = length(p/(r*r));
  return k0*(k0-1.)/k1;
}
float cap(vec3 p, vec3 a, vec3 b, float r) {
  vec3 pa = p-a, ba = b-a;
  float h = clamp(dot(pa,ba)/dot(ba,ba), .0, 1.);
  return length(pa-ba*h)-r;
}
float diss(vec3 p, float s) {
    float d1 = sphere(p, s); 
    float d2 = (1.+sin(frame/100.))/2.*.05*(sin(15.*p.x)*cos(15.*p.y)*sin(15.*p.z));
    return d1+d2;
}

vec2 sdf(in vec3 p) {
    vec3 pos = vec3(.0,sin(frame/30.)*.5,.0);
    vec2 r = vec2(diss(p-pos,1.),.0);
    r = uni(r, vec2(sphere(p-vec3(-.3,.3,1.5)-pos, .15), 1.));
    r = uni(r, vec2(sphere(p-vec3(.3,.3,1.5)-pos, .15), 1.));
    r = uni(r, vec2(sphere(p-vec3(-.3,.3,1.7)-pos, .05), 2.));
    r = uni(r, vec2(sphere(p-vec3(.3,.3,1.7)-pos, .05), 2.));
    r = uni(r, vec2(torus(p-vec3(.0,-.3,1.3)-pos, vec2(max(.05, (1.+sin(frame/100.))*.2)/2., .03)), 3.));
    r = suni(r, vec2(elli(p-vec3(.9*(1.+sin(frame/60.)/4.),-.9*(1.2-sin(frame/30.)/6.),0.5)-pos, vec3(.2,.3,.4)), 0.), .7);
    r = suni(r, vec2(elli(p-vec3(-.9*(1.+sin(frame/60.)/4.),-.9*(1.2-sin(frame/30.)/6.),0.5)-pos, vec3(.2,.3,.4)), 0.), .7);
    r = uni(r, vec2(cap(p-vec3(.1,.5,1.0)-pos, vec3(.1,.1,1.0), vec3(.35,.05,1.0), .03), 2.));
    r = uni(r, vec2(cap(p-vec3(-.1,.5,1.0)-pos, vec3(-.1,.1,1.0), vec3(-.35,.05,1.0), .03), 2.));
    return r;
}

vec2 march(vec3 eye, vec3 dir, float s, float e) {
    float d = s;
    for (float i = 0.; i < MAX_STEPS; i++) {
        vec2 res = sdf(eye + d * dir);
        steps = i;
        if (res.x < EPS) return vec2(d, res.y);
        d += res.x;
        if (d >= e) return vec2(e, .0);
    }
    return vec2(e, .0);
}

vec3 rayDir(float fov, vec2 uv) {
    vec2 xy = uv * 2. - 1.;
    xy.y = xy.y / (16. / 9.);
    float z = 2. / tan(radians(fov / 2.));
    return normalize(vec3(xy, -z));
}

vec3 estimateNormal(vec3 p) {
    return normalize(vec3(
        sdf(vec3(p.x + EPS, p.yz)).x - sdf(vec3(p.x - EPS, p.yz)).x,
        sdf(vec3(p.x, p.y + EPS, p.z)).x - sdf(vec3(p.x, p.y - EPS, p.z)).x,
        sdf(vec3(p.xy, p.z + EPS)).x - sdf(vec3(p.xy, p.z - EPS)).x
    ));
}

vec3 phongContribForLight(vec3 k_d, vec3 k_s, float alpha, vec3 p, vec3 eye, vec3 lightPos, vec3 lightIntensity) {
    vec3 N = estimateNormal(p);
    vec3 L = normalize(lightPos - p);
    vec3 V = normalize(eye - p);
    vec3 R = normalize(reflect(-L, N));
    float dotLN = dot(L, N);
    float dotRV = dot(R, V);
    if (dotLN < 0.) return vec3(0., 0., 0.);
    if (dotRV < 0.) return lightIntensity * (k_d * dotLN);
    return lightIntensity * (k_d * dotLN + k_s * pow(dotRV, alpha));
}

vec3 phongIllumination(vec3 k_a, vec3 k_d, vec3 k_s, float alpha, vec3 p, vec3 eye) {
    const vec3 amb = .5 * vec3(1., 1., 1.);
    vec3 color = amb * k_a;
    vec3 pos1 = vec3(.0, 1.*frame/100., 30.*frame/2000.);
    vec3 ints1 = vec3(.4, .4, .4);
    vec3 phong = phongContribForLight(k_d, k_s, alpha, p, eye, pos1, ints1);
    color += phong;
    return color;
}

void main() {
  vec3 eye = vec3(.0,.0,10.);
  vec3 dir = rayDir(60.0, vUv);
  vec2 res = march(eye, dir, START, END);
  vec3 color = vec3(.0);
  if (res.x >= END-EPS) {
    float glow = min(max(0.,(steps-15.)*.08), 1.);
    gl_FragColor = vec4(vec3(glow), 1.0);
    return;
  }
  vec3 p = eye + dir * res.x;
  color = vec3(vUv, .5+.5*sin(frame/60.)); 
  if(res.y == 1.) color = vec3(.9);
  if(res.y == 2.) color = vec3(.0);
  if(res.y == 3.) color = vec3(.9,.0,.0);
  color = phongIllumination(color, color, normalize(vec3(1., 1., 1.)), 10., p, eye);
  gl_FragColor = vec4(color, 1.);
}
