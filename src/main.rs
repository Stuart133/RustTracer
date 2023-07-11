mod camera;
mod hittable;
mod material;
mod math;
mod objects;
mod ray;

use std::rc::Rc;

use crate::{camera::Camera, hittable::HittableList, math::Color, math::Point, objects::Sphere};

// Quick hack to avoid floating point uncertainty causing self intersections
const MIN_INTERSECTION_DISTANCE: f64 = 0.000000001;

const SAMPLES_PER_PIXEL: i64 = 100;
const MAX_DEPTH: i64 = 50;
const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: i64 = 400;
const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&world, MAX_DEPTH);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("Done");
}

fn write_color(pixel_color: Color, samples_per_pixel: i64) {
    let scale = 1.0 / samples_per_pixel as f64;

    // Average pixel samples and perform a quick gamma correction
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u64,
        (256.0 * g.clamp(0.0, 0.999)) as u64,
        (256.0 * b.clamp(0.0, 0.999)) as u64
    );
}
