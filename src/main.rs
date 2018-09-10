mod write;
use write::write_png;

mod scene;
use scene::Scene;

mod vector;
use vector::{Ray, Vec3};

mod sphere;
use sphere::Sphere;

mod plane;
use plane::Plane;

use std::{f64,ptr};

fn render(image: &mut Vec<u8>, size_x: u32, size_y: u32, scene: &Scene) {
    image[0] = 0;

    let mut r = Ray::default_ray();
    let view_x = 1.0;
    let view_y = 1.0;

    let offset_x = [-0.333, 0.0, 0.333, -0.333, 0.0, 0.333, -0.333, 0.0, 0.333];
    let offset_y = [-0.333, -0.333, -0.333, 0.0, 0.0, 0.0, 0.333, 0.333, 0.333];

    for x in 0..size_x {
        for y in 0..size_y {
            let mut color = Vec3::default_vec();
            for i in 0..offset_x.len() {
                r.dir.v[0] = ((x as f64 + offset_x[i]) / size_x as f64 - 0.5) * view_x;
                r.dir.v[1] = ((y as f64 + offset_y[i]) / size_y as f64 - 0.5) * view_y;
                r.dir.v[2] = -1.0;

                r.normalize();

                color = color + scene.intersect(&r, true, ptr::null()).1;
            }
            color = color / 9.0;
            image[((y * size_x + x) * 4 + 0) as usize] = (color.v[0] * 255.0) as u8;
            image[((y * size_x + x) * 4 + 1) as usize] = (color.v[1] * 255.0) as u8;
            image[((y * size_x + x) * 4 + 2) as usize] = (color.v[2] * 255.0) as u8;
            image[((y * size_x + x) * 4 + 3) as usize] = 255;
        }
    }
}

fn main() {
    let mut f = std::fs::File::create("test.png").unwrap();

    // image from bottom to top 3x2
    let image_width = 800u32;
    let image_height = 800u32;
    let mut image = vec![255; (image_width * image_height * 4) as usize];

    {
        let mut scene = Scene::default_scene();

        scene.add(Box::new(Sphere {
            c: Vec3 {
                v: [0.3, 0.0, -1.5],
            },
            r: 0.3,
            col: Vec3 { v: [0.8, 0.7, 0.0] },
        }));
        scene.add(Box::new(Sphere {
            c: Vec3 {
                v: [-0.2, 0.4, -2.0],
            },
            r: 0.3,
            col: Vec3 {
                v: [0.5, 0.6, 0.40],
            },
        }));
        scene.add(Box::new(Plane {
            pos: Vec3 {
                v: [0.0, -0.35, 0.0],
            },
            norm: Vec3 { v: [0.0, 1.0, 0.0] },
            col: Vec3 { v: [0.8, 0.8, 0.8] },
        }));

        render(&mut image, image_width, image_height, &scene);
    }

    match write_png(&mut f, &image, image_width, image_height) {
        Ok(_) => println!("Written image!"),
        Err(e) => println!("Error {:?}", e),
    }
}
