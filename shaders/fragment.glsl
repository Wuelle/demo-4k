#version 330 core

uniform float u_time;

const vec2 s = vec2(.866025, 1);
vec2 center;
const vec3 k = vec3(-0.866025404,0.5,0.577350269);
const float ambient_light = 0.1;
const vec3 light_color = vec3(1., 1., 1.);
const float pi = 3.14;

float circle(vec2 center, float wave_width, float offset) {
  float mix_factor = fract((length(center) - offset) / wave_width);

  // Make the wave symmetric
  return smoothstep(0, 1, min(mix_factor, 1. - mix_factor));
}

float wave_factor(vec2 center) {
  float blend_in_start = 20.;
  float blend_in_end = 27.;
  return circle(center, 5., u_time * 3) * smoothstep(blend_in_start, blend_in_end, u_time);
}

float wave_factor_2(vec2 center) {
  float blend_in_start = 34.;
  float blend_in_end = 39.;

  float angle = abs(atan(center.y, center.x) + u_time /2.);
  float pi_over_4 = pi/4.;
  float angle_factor = mod(angle, pi_over_4) / pi_over_4;
  angle_factor = min(angle_factor, 1. - angle_factor);
  return min(3. * circle(center, 10., 3.) * smoothstep(0, 1, angle_factor), 1.) * smoothstep(blend_in_start, blend_in_end, u_time);
}

float hex_height(vec2 center){
    return dot(sin(center * 2. - cos(center.yx*1.4)), vec2(.17)) + 2.5 + 0.2 * wave_factor(center);
}

float hexagon(vec2 p, float r)
{
    p = abs(p);
    p -= 2.0 * min(dot(k.xy,p), 0.0) * k.xy;
    p -= vec2(clamp(p.x, -k.z * r, k.z * r), r);
    return length(p) * sign(p.y);
}

float hexagon3d(vec2 b, float ph, float r, float h) {
  vec3 p = vec3(b.x, ph, b.y);
  float horizontal_distance = hexagon(b, r);
  vec2 w = vec2( horizontal_distance, abs(p.y) - h );
  return min(max(w.x,w.y),0.0) + length(max(w,0.0));
}

float smooth_hex(vec2 b, float ph, float r, float h) {
  // Smoothness
  return hexagon3d(b, ph, r, h) - 0.02;
}

float objDist(in vec2 p, float pH, float r, vec2 id){
  return smooth_hex(p, pH, r, hex_height(id));  
}

vec3 get_nearest_hex(in vec2 p, float pH, float pylon_radius){ 
    vec4 center1 = floor(vec4(p, p - vec2(0, .5))/s.xyxy) + vec4(0, 0, 0, .5);
    vec4 center2 = floor(vec4(p - vec2(.5, .25), p - vec2(.5, .75))/s.xyxy) + vec4(.5, .25, .5, .75);
    
    // Centering the coordinates with the hexagon centers above.
    vec4 h = vec4(p - (center1.xy + .5)*s, p - (center1.zw + .5)*s);
    vec4 h2 = vec4(p - (center2.xy + .5)*s, p - (center2.zw + .5)*s);

    vec4 obj = vec4(
      objDist(h.xy, pH, pylon_radius, center1.xy), 
      objDist(h.zw, pH, pylon_radius, center1.zw), 
      objDist(h2.xy, pH, pylon_radius, center2.xy), 
      objDist(h2.zw, pH, pylon_radius, center2.zw)
    );

    // Nearest hexagon center (with respect to p) to the current point. In other words, when
    // "h.xy" is zero, we're at the center. We're also returning the corresponding hexagon ID -
    // in the form of the hexagonal central point.
    vec3 first_center = obj.x < obj.y ? vec3(obj.x, center1.xy) : vec3(obj.y, center1.zw);
    vec3 second_center = obj.z < obj.w ? vec3(obj.z, center2.xy) : vec3(obj.w, center2.zw);
    
    return first_center.x < second_center.x ? first_center : second_center;  
}

float map(vec3 p){   
    vec3 h = get_nearest_hex(p.xz, p.y, 0.23);

    center = h.yz;
    return h.x;
}

vec3 prism_color(){
  vec3 natural_color = vec3(0.8 + sin(center.x * 5) / 5., 0.2, 0.6 + cos(center.y * 5) / 5.);
  vec3 effect_color = vec3(1., 0., 0.);
  vec3 effect_color_2 = vec3(0., 0., 1.);
  vec3 after_effect_1 = mix(natural_color, effect_color, wave_factor(center));
  return mix(after_effect_1, effect_color_2, wave_factor_2(center));
}

vec3 getNormal(vec3 p) {
  const vec2 e = vec2(0.0025, 0);
  return normalize(vec3(map(p + e.xyy) - map(p - e.xyy), map(p + e.yxy) - map(p - e.yxy), map(p + e.yyx) - map(p - e.yyx)));
}

const float MAX_DEPTH = 60.0;

// Returns (x, y) where x is the hit parameter t and
// y is the closest miss of scene geometry (useful for shadows)
vec2 march(vec3 ro, vec3 rd, float max_t) {
  float t = 0.0;
  float nearest_miss = 1.;
  float previous_distance = 1e20;

  for(int i = 0; i < 200 && t < max_t-0.1; i++){
    float distance = map(ro + t * rd);
    if (distance < .001) return vec2(t + distance, ambient_light);

     float y = distance * distance /(2.0*previous_distance);
    float d = sqrt(distance * distance - y * y);
     nearest_miss = min(nearest_miss, 30. * distance / t);
    // nearest_miss = min(nearest_miss, 300 * d / min(0., t-y) );
    previous_distance = distance;

    t += distance;
  }

  // nearest_miss = nearest_miss*nearest_miss*(3.0-2.0*nearest_miss);
  return vec2(t, 1.);
} 

void main(){
  vec2 resolution = vec2(1000., 600.);
  vec2 uv = (gl_FragCoord.xy - resolution.xy / 2.) / resolution.y;
  vec3 light_position = vec3(3., 10., 3.);

  vec3 ro; // Ray origin
  vec3 lk; // Look at

  float scene2 = 15;
  float scene3 = 40;
  float scene4 = 65;
  if (u_time < scene2) {
    ro = vec3(5. + u_time, 8., 5.);
    lk = ro + vec3(1. + sin(u_time / 3.), -1.5, 2.);
  } else if (u_time < scene3) {
    ro = vec3(5., u_time - 6., 5.);
    lk = vec3(0.); 
  } else if (u_time < scene4) {
    ro = vec3(5., 2. * scene3 - u_time - 6., 5.);
    lk = vec3(u_time - scene3, u_time - scene3, 0.);
  } else {
    ro = vec3(5., 9., 5.);
    lk = vec3(25., 25., 0.);
  }

  // Rotate the camera
  // This is stupid, but it works!
  float camera_angle = 0.;
  if (u_time > 67) {
    camera_angle = pi * smoothstep(67., 72., u_time);
  }
  mat2 rot = mat2(cos(camera_angle), -sin(camera_angle), sin(camera_angle), cos(camera_angle));
  uv = rot * uv;

  float FOV = 3.14159 / 2.5;
  vec3 forward = normalize(lk - ro);
  vec3 right = normalize(vec3(forward.z, 0., -forward.x )); 
  vec3 up = cross(forward, right);

  vec3 rd = normalize(forward + FOV * uv.x * right + FOV * uv.y * up);
  rd.y = -abs(rd.y);

  //Raymarching
  float t = march(ro, rd, MAX_DEPTH).x;
  vec3 p = ro + t * rd;

  if (t < MAX_DEPTH){
    // Hit
    vec3 n = getNormal(p);

    float b = dot(n, -rd);
    float l = dot(n, -normalize(light_position - p));
    vec3 natural_color = prism_color();


    vec3 pc = p - light_position;
    vec2 cam_intersection = march(light_position, normalize(pc), length(pc));
    float light_strength = cam_intersection.y;
    // light_strength *= (1.0 - length(pc) * .05); // This should be quadratic...

    //simple phong LightPosition=light_positionPosition
    
    float specular = pow(l, 8.0) / 8.;
    vec3 color = b * natural_color;
    color += specular * light_color;

    // Fog
    color *= exp( -0.000015*t*t*t );

    gl_FragColor = vec4(color * light_strength, 1.0);
  }
  else gl_FragColor=vec4(0.,0,0,1); //background color
}