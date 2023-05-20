const CW: i32 = 600;
const CH: i32 = 600;
const TMAX: f32 = f32::MAX;

fn main() {
    let scene: Scene = Scene {
        spheres: Vec::from([
            Sphere {
                center: [0.0, -1.0, 3.0],
                radius: 1.0,
                color: [255, 0, 0],
            },
            Sphere {
                center: [2.0, 0.0, 4.0],
                radius: 1.0,
                color: [0, 0, 255],
            },
            Sphere {
                center: [-2.0, 0.0, 4.0],
                radius: 1.0,
                color: [0, 255, 0],
            }
        ])
    };

    let mut canvas: image::RgbImage = image::ImageBuffer::new(CW as u32, CH as u32);
    let v: Viewport = Viewport { d: 1.0, width: 1.0, height: 1.0 };
    let origin: [f32; 3] = [0.0, 0.0, 0.0];

    for cx in (-CW/2)..(CW/2) {
        for cy in (-CH/2)..(CH/2) {
            let d: [f32; 3] = v.canvas_to_viewport(cx, cy);
            let color = trace_ray(origin, d, v.d, &scene);

            let sx: u32 = ((CW / 2) + cx) as u32;
            let sy: u32 = ((CH / 2) - cy - 1) as u32;

            canvas.put_pixel(sx, sy, image::Rgb(color));

        }
    }

    canvas.save("image.png").unwrap();
}

struct Scene {
    spheres: Vec<Sphere>,
}

struct Sphere {
    center: [f32; 3],
    radius: f32,
    color: [u8; 3],
}

struct Viewport {
    d: f32,
    width: f32,
    height: f32,
}

impl Viewport {
    fn canvas_to_viewport(&self, cx: i32, cy: i32) -> [f32; 3] {
        return [
            cx as f32 * (self.width / CW as f32), 
            cy as f32 * (self.height / CH as f32), 
            self.d
        ]
    }
}

fn trace_ray(origin: [f32; 3], direction: [f32; 3], t_min: f32, scene:&Scene) -> [u8; 3] {
    let mut closest_t: f32 = TMAX;
    let mut closest_sphere: &Sphere = &Sphere { center: [0.0, 0.0, 0.0], radius: 0.0, color: [255, 255, 255] };

    for sphere in &scene.spheres {
        let (t1, t2): (f32, f32) = intersect_ray_sphere(origin, direction, sphere);

        if t_min < t1 && t1 < TMAX && t1 < closest_t {
            closest_t = t1;
            closest_sphere = sphere;
        }
        if t_min < t2 && t2 < TMAX && t2 < closest_t {
            closest_t = t2;
            closest_sphere = sphere;
        }

    }
    
    return closest_sphere.color
}

fn intersect_ray_sphere(origin: [f32; 3], direction: [f32; 3], sphere: &Sphere) -> (f32, f32) {
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