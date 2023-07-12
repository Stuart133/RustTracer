mod camera;
mod hittable;
mod material;
mod math;
mod objects;
mod ray;

use crate::{
    camera::Camera,
    hittable::HittableList,
    math::Point,
    math::{Color, Vector},
};

use rayon::prelude::*;

// Quick hack to avoid floating point uncertainty causing self intersections
const MIN_INTERSECTION_DISTANCE: f64 = 0.0001;

const THREADS: u64 = 10;
const SAMPLES_PER_PIXEL: u64 = 500;
const SAMPLES_PER_PIXEL_PER_THREAD: u64 = SAMPLES_PER_PIXEL / THREADS;
const MAX_DEPTH: i64 = 50;

const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: i64 = 1200;
const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

fn main() {
    // World
    let world = HittableList::random_scene();

    // Camera
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
    );

    // Render
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let images: Vec<Vec<Color>> = (0..THREADS)
        .into_par_iter()
        .map(|t| {
            let mut image = vec![];

            for j in (0..IMAGE_HEIGHT).rev() {
                eprintln!("Thread {t}, scanlines remaining: {j}");
                for i in 0..IMAGE_WIDTH {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..SAMPLES_PER_PIXEL_PER_THREAD {
                        let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                        let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                        let ray = camera.get_ray(u, v);
                        pixel_color += ray.color(&world, MAX_DEPTH);
                    }

                    image.push(average_pixel(pixel_color, SAMPLES_PER_PIXEL_PER_THREAD))
                }
            }

            image
        })
        .collect();

    for pixel in images[0].iter() {
        write_color(*pixel, 1);
    }

    eprintln!("Done");
}

fn average_pixel(pixel_color: Color, samples_per_pixel: u64) -> Color {
    let scale = 1.0 / samples_per_pixel as f64;

    // Average pixel samples and perform a quick gamma correction
    Color::new(
        (pixel_color.x * scale).sqrt(),
        (pixel_color.y * scale).sqrt(),
        (pixel_color.z * scale).sqrt(),
    )
}

fn write_color(pixel_color: Color, samples_per_pixel: u64) {
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
