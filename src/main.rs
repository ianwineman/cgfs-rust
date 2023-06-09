mod raytracer;
use crate::raytracer::*;

fn main() {
    let scene: Scene = Scene {
        spheres: Vec::from([
            Sphere {
                center: [0.0, -1.0, 3.0],
                radius: 1.0,
                color: [255, 0, 0],
                specular: 500.0,
            },
            Sphere {
                center: [2.0, 0.0, 4.0],
                radius: 1.0,
                color: [0, 0, 255],
                specular: 500.0,
            },
            Sphere {
                center: [-2.0, 0.0, 4.0],
                radius: 1.0,
                color: [0, 255, 0],
                specular: 10.0,
            },
            Sphere {
                center: [0.0, -5001.0, 0.0],
                radius: 5000.0,
                color: [255, 255, 0],
                specular: 1000.0,
            }
        ]),
        lights: Vec::from([
            Light::Ambient { intensity: 0.2 },
            Light::Point { intensity: 0.6, position: [2.0, 1.0, 0.0] },
            Light::Directional { intensity: 0.2, direction: [1.0, 4.0, 4.0] }
        ])
    };

    let canvas: image::RgbImage = image::ImageBuffer::new(CW as u32, CH as u32);
    let viewport: Viewport = Viewport { d: 1.0, width: 1.0, height: 1.0 };
    let origin: [f32; 3] = [0.0, 0.0, 0.0];

    render_scene(canvas, scene, viewport, origin);
}