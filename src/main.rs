mod hittable;
mod objects;
mod ray;

use std::rc::Rc;

use nalgebra::{Point3, Vector3};

use crate::{hittable::HittableList, objects::Sphere, ray::Ray};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: i64 = 400;
const IMAGE_HEIGHT: i64 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i64;

type Vector = Vector3<f64>;
type Color = Vector3<f64>;
type Point = Point3<f64>;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vector::new(viewport_width, 0.0, 0.0);
    let vertical = Vector::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vector::new(0.0, 0.0, focal_length);

    // Render
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray.color(&world);

            write_color(pixel_color);
        }
    }

    eprintln!("Done");
}

fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * pixel_color.x) as u64,
        (255.999 * pixel_color.y) as u64,
        (255.999 * pixel_color.z) as u64
    );
}
