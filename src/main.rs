mod raytracer;
use crate::raytracer::*;

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

    let canvas: image::RgbImage = image::ImageBuffer::new(CW as u32, CH as u32);
    let viewport: Viewport = Viewport { d: 1.0, width: 1.0, height: 1.0 };
    let origin: [f32; 3] = [0.0, 0.0, 0.0];

    render_scene(canvas, scene, viewport, origin);
}