pub const CW: i32 = 600;
pub const CH: i32 = 600;
const TMAX: f32 = f32::MAX;
const BACKGROUNDCOLOR: [u8; 3] = [255, 255, 255];

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [u8; 3],
}

pub struct Viewport {
    pub d: f32,
    pub width: f32,
    pub height: f32,
}

impl Viewport {
    pub fn canvas_to_viewport(&self, cx: i32, cy: i32) -> [f32; 3] {
        return [
            cx as f32 * (self.width / CW as f32), 
            cy as f32 * (self.height / CH as f32), 
            self.d
        ]
    }
}

pub enum Light {
    Point { intensity: f32, position: [f32; 3] },
    Directional { intensity: f32, direction: [f32; 3] },
    Ambient { intensity: f32 },
}

pub fn trace_ray(origin: [f32; 3], direction: [f32; 3], t_min: f32, scene:&Scene) -> [u8; 3] {
    let mut closest_t: f32 = TMAX;
    let mut closest_sphere: &Sphere = &Sphere { center: [0.0, 0.0, 0.0], radius: 0.0, color: BACKGROUNDCOLOR };
    let mut background: bool = true;

    for sphere in &scene.spheres {
        let (t1, t2): (f32, f32) = intersect_ray_sphere(origin, direction, sphere);

        if t_min < t1 && t1 < TMAX && t1 < closest_t {
            closest_t = t1;
            closest_sphere = sphere;
            background = false;
        }
        if t_min < t2 && t2 < TMAX && t2 < closest_t {
            closest_t = t2;
            closest_sphere = sphere;
            background = false;
        }

    }

    if !background {
        let point: [f32; 3] = [
            origin[0] + (closest_t * direction[0]),
            origin[1] + (closest_t * direction[1]),
            origin[2] + (closest_t * direction[2])
        ];
        let normal: [f32; 3] = [
            point[0] - closest_sphere.center[0],
            point[1] - closest_sphere.center[1],
            point[2] - closest_sphere.center[2]
        ];
        let normal_len: f32 = (normal[0] * normal[0]) + (normal[1] * normal[1]) + (normal[2] * normal[2]).sqrt();
        let unit_normal: [f32; 3] = [
            normal[0] / normal_len,
            normal[1] / normal_len,
            normal[2] / normal_len
        ];
        let lighting: f32 = compute_lighting(point, unit_normal, &scene);

        return [
            (closest_sphere.color[0] as f32 * lighting).round() as u8,
            (closest_sphere.color[1] as f32 * lighting).round() as u8,
            (closest_sphere.color[2] as f32 * lighting).round() as u8
        ]
    }
    else {
        return closest_sphere.color
    }
}

pub fn intersect_ray_sphere(origin: [f32; 3], direction: [f32; 3], sphere: &Sphere) -> (f32, f32) {
    let r: f32 = sphere.radius;
    let co: [f32; 3] = [origin[0] - sphere.center[0], origin[1] - sphere.center[1], origin[2] - sphere.center[2]];

    let a: f32 = (direction[0] * direction[0]) + (direction[1] * direction[1]) + (direction[2] * direction[2]);
    let b: f32 = 2.0 * ((co[0] * direction[0]) + (co[1] * direction[1]) + (co[2] * direction[2]));
    let c: f32 = (co[0] * co[0]) + (co[1] * co[1]) + (co[2] * co[2]) - (r * r);

    let discriminant: f32 = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return (TMAX, TMAX)
    }

    let t1: f32 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2: f32 = (-b - discriminant.sqrt()) / (2.0 * a);

    return (t1, t2)
}

pub fn render_scene(mut canvas: image::RgbImage, scene: Scene, viewport: Viewport, origin: [f32; 3]) {
    for cx in (-CW/2)..(CW/2) {
        for cy in (-CH/2)..(CH/2) {
            let d: [f32; 3] = viewport.canvas_to_viewport(cx, cy);
            let color = trace_ray(origin, d, viewport.d, &scene);

            let sx: u32 = ((CW / 2) + cx) as u32;
            let sy: u32 = ((CH / 2) - cy - 1) as u32;

            canvas.put_pixel(sx, sy, image::Rgb(color));

        }
    }

    canvas.save("image.png").unwrap();
}

pub fn compute_lighting(point: [f32; 3], normal: [f32; 3], scene: &Scene) -> f32 {
    let mut total_intensity: f32 = 0.0;

    for light in &scene.lights {
        match light {
            Light::Ambient { intensity } => total_intensity += intensity,
            Light::Point { intensity, position } => {
                let l: [f32; 3] = [position[0] - point[0], position[1] - point[1], position[2] - point[2]];
                let normal_dot_l: f32 = (normal[0] * l[0]) + (normal[1] * l[1]) + (normal[2] * l[2]);

                if normal_dot_l > 0.0 {
                    total_intensity += intensity * (normal_dot_l / (
                        (normal[0] * normal[0]) + (normal[1] * normal[1]) + (normal[2] * normal[2]).sqrt() *
                        (l[0] * l[0]) + (l[1] * l[1]) + (l[2] * l[2]).sqrt()
                    ))
                }
            }
            Light::Directional { intensity, direction } => {
                let l: [f32; 3] = *direction;
                let normal_dot_l: f32 = (normal[0] * l[0]) + (normal[1] * l[1]) + (normal[2] * l[2]);

                if normal_dot_l > 0.0 {
                    total_intensity += intensity * (normal_dot_l / (
                        (normal[0] * normal[0]) + (normal[1] * normal[1]) + (normal[2] * normal[2]).sqrt() *
                        (l[0] * l[0]) + (l[1] * l[1]) + (l[2] * l[2]).sqrt()
                    ))
                }
            }
        }
    }

    return total_intensity
}