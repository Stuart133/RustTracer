mod aabb;
mod bvh;
mod camera;
mod hittable;
mod instance;
mod material;
mod math;
mod objects;
mod perlin;
mod ray;
mod rectangle;
mod scene;
mod texture;

use std::sync::Arc;

use crate::{bvh::BVHNode, hittable::HittableList, math::Color, math::Point};

use rayon::prelude::*;

// Quick hack to avoid floating point uncertainty causing self intersections
const MIN_INTERSECTION_DISTANCE: f64 = 0.0001;

const THREADS: u64 = 16;
const MAX_DEPTH: i64 = 50;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

fn main() {
    // Scene
    let scene = scene::cornell_box();
    let samples_per_pixel_per_thread = scene.image.samples_per_pixel / THREADS;

    // BVH
    let bvh = BVHNode::new(scene.objects, 0.0, 1.0);
    let mut world = HittableList::new();
    world.add(Arc::new(bvh));

    // Render
    println!("P3");
    println!("{} {}", scene.image.width, scene.image.height);
    println!("255");

    let images: Vec<Vec<Color>> = (0..THREADS)
        .into_par_iter()
        .map(|t| {
            let mut image = vec![];

            for j in (0..scene.image.height).rev() {
                if j % 10 == 0 {
                    eprintln!("Thread {t}, scanlines remaining: {j}");
                }
                for i in 0..scene.image.width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                    for _ in 0..samples_per_pixel_per_thread {
                        let u = (i as f64 + rand::random::<f64>()) / (scene.image.width - 1) as f64;
                        let v =
                            (j as f64 + rand::random::<f64>()) / (scene.image.height - 1) as f64;
                        let ray = scene.camera.get_ray(u, v);
                        pixel_color += ray.color(scene.background, &world, MAX_DEPTH);
                    }

                    image.push(average_pixel(pixel_color, samples_per_pixel_per_thread))
                }
            }

            image
        })
        .collect();

    for i in 0..images[0].len() {
        // Add up all the values for each pixel in each image
        let color = images
            .iter()
            .fold(Color::new(0.0, 0.0, 0.0), |acc, image| acc + image[i]);

        write_pixel(average_pixel(color, THREADS));
    }

    eprintln!("Done");
}

fn average_pixel(pixel_color: Color, samples_per_pixel: u64) -> Color {
    let scale = 1.0 / samples_per_pixel as f64;

    Color::new(
        pixel_color.x * scale,
        pixel_color.y * scale,
        pixel_color.z * scale,
    )
}

fn write_pixel(pixel_color: Color) {
    // Perform gamma correction
    let r = pixel_color.x.sqrt();
    let g = pixel_color.y.sqrt();
    let b = pixel_color.z.sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as u64,
        (256.0 * g.clamp(0.0, 0.999)) as u64,
        (256.0 * b.clamp(0.0, 0.999)) as u64
    )
}
