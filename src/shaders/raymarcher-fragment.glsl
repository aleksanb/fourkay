uniform float frame;
uniform vec3 eye;
uniform vec3 forward;
uniform vec2 resolution;

float sphere(vec3 position, vec3 sphere, float radius)
{
  return length(position - sphere) - radius;
}

float box(vec3 position, vec3 box, vec3 dimensions)
{
  return length(max(abs(position - box) - dimensions, 0.0));
}

float distance(vec3 p)
{
  //p.x = mod(p.x, 5.0) - 2.5;
  //p.z = mod(p.z, 5.0) - 2.5;

  //float s = sphere(p, vec3(.0, .0, .0), 2.0);
  float b = box(p, vec3(3.0, 0.1, -10.0), vec3(2.0));

  return b;
}

vec2 castRay(vec3 ro, vec3 rd)
{
  float totalDistance = 0.0;
  const int maxSteps = 64;

  for(int i = 0; i < maxSteps; ++i)
  {
    vec3 p = ro + rd * totalDistance;
    float d = distance(p);
    if(d < 0.01 || totalDistance >= 40.0)
    {
      break;
    }

    totalDistance += d;
  }

  float material = 1.0;
  if (totalDistance >= 40.0) {
    material = 0.0;
  }

  return vec2(totalDistance, material);
}

vec3 calculateNormal(vec3 pos) {
  vec3 eps = vec3(0.001, 0.0, 0.0);
  vec3 normal = vec3(
    distance(pos + eps.xyy) - distance(pos - eps.xyy),
    distance(pos + eps.yxy) - distance(pos - eps.yxy),
    distance(pos + eps.yyx) - distance(pos - eps.yyx));

  return normalize(normal);
}

void main()
{
  vec2 position = gl_FragCoord.xy / resolution;
  float x = (position.x * 16.0) - 8.0;// + sin(frame / 30.0);
  float y = (position.y * 9.0) - 4.5;// - cos(frame / 40.0);
  float fov = 9.0;

  vec3 up = vec3(0.0, 1.0, 0.0);
  vec3 right = cross(forward, up);

  vec3 rayOrigin = eye + (right * x) + (up * y) + (forward * fov);
  vec3 rayDestination = normalize(rayOrigin - eye);

  vec3 light = normalize(
    vec3(
      5.0 * sin(frame / 60.0),
      5.0 * sin(sin(frame / 40.0)),
      5.0 * cos(frame / 50.0)));

  float farClippingPlane = 100.0;
  vec2 result = castRay(rayOrigin, rayDestination);
  float distance = result.x;
  float material = result.y;

  vec4 color = vec4(1.0, 0.5, 0.1, 1.0);
  if (material > 0.0)
  {
    // Surface normal
    vec3 pos = rayOrigin + forward * distance;
    vec3 surfaceNormal = calculateNormal(pos);
    float diffusion = 1.5 * clamp(dot(surfaceNormal, light), 0.0, 1.0);

    color += diffusion * vec4(0.9, 0.5, 0.5, 1.0);
    if (distance > farClippingPlane) {
      color = vec4(0.0);
    }
  }

  vec4 fog = vec4(0.5, 0.6, 0.7, 1.0);
  float mixer =  1.0 - exp(-distance * 0.07);
  color = mix(color, fog, mixer);

  gl_FragColor = color;
}