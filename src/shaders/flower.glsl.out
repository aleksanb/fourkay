#extension GL_OES_standard_derivatives : enable

uniform float f;
uniform vec2 r;

float sl(in vec2 p,in vec2 a,in vec2 b) {
  vec2 pa=p-a, ba=b-a;
  float h=clamp(dot(pa, ba)/dot(ba, ba),.0,1.);
  return length(pa-ba*h);
}
vec3 l(in vec3 q,in vec2 a,in vec2 b,in vec2 p,in vec2 w,in vec4 c) {
  float f=sl(p,a,b);
  float g=fwidth(f)*w.y;
  return mix(q,c.xyz,c.w*(1.-smoothstep(w.x-g,w.x+g,f)));
}
vec3 h(float n){return fract(sin(vec3(n,n+1.,n+2.))*43758.5453123);}
void main() {
  vec2 uv = gl_FragCoord.xy / r;
  uv-=vec2(.5,.5);
  uv/=vec2(9./16.,1.)*max(smoothstep(1.2,.1,(f-16.*60.)/600.),.07);
  float pi = 3.1415926;
  float tau = 2.*pi;
  float space = 8.;
  float speed = 60.*space;
  float r = length(uv);
  vec3 col=vec3(.0,.0,.0);
  vec2 dir;
  vec2 q1=mod(uv,vec2(2.5,2.5))-.5*vec2(2.5,2.5);
  vec2 q2=mod(uv+2.5,vec2(5.,5.))-.5*vec2(5.,5.);
  float j=.0;
  for(int i=0;i<12;i++){
    dir=vec2(sin(tau*(f+j)/speed),cos(tau*(f+j)/speed));
    col=l(col,vec2(.0),dir*.7,q1,vec2(.015,1.),vec4(173./255.,127./255.,1.,1.));
    j+=5.*space;
  }
  j=.0;
  for(int i=0;i<12;i++){
    dir=vec2(cos(tau*(f+j)/speed),sin(tau*(f+j)/speed));
    col=l(col,vec2(.0),dir*.4,q2,vec2(.015,1.),vec4(1., 127./255.,1.,1.));
    j+=5.*space;
  }
  j=5.*space/2.;
  for(int i=0;i<12;i++){
    dir=vec2(cos(tau*(f+j)/speed), sin(tau*(f+j)/speed));
    col=l(col,vec2(.0),dir*.9,q2,vec2(.005,1.),vec4(85./255.,127./255.,1.,1.));
    j+=5.*space;
  }
  col=mix(col,vec3(133./255.,243./255.,159./255.),1.-smoothstep(.05,.055,r*smoothstep(.005,1.,(f-16.*60.)/600.)));
  col+=(1./255.)*h(uv.x+13.*uv.y)/.5;
  gl_FragColor=vec4(col,1.);
}
