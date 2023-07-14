#![allow(dead_code)]
use std::sync::Arc;

use crate::{
    camera::Camera,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    math::{random_color, random_range, Color, Point, Vector},
    objects::{MovingSphere, Sphere},
    texture::CheckerTexture,
    ASPECT_RATIO,
};

pub struct Scene {
    pub objects: HittableList,
    pub camera: Camera,
}

pub fn weekend_scene(n: i64) -> Scene {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -n..n {
        for b in -n..n {
            let choose_material: f64 = rand::random();
            let center = Point::new(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                match choose_material {
                    x if x < 0.8 => {
                        // Diffuse
                        let end_center: nalgebra::OPoint<f64, nalgebra::Const<3>> =
                            center + Vector::new(0.0, random_range(0.0, 0.5), 0.0);
                        world.add(Arc::new(MovingSphere::new(
                            center,
                            end_center,
                            0.0,
                            1.0,
                            0.2,
                            Arc::new(Lambertian::new_from_color(random_color(0.0, 1.0))),
                        )))
                    }
                    x if x < 0.95 => {
                        // Metal
                        world.add(Arc::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Metal::new(random_color(0.5, 1.0), random_range(0.0, 0.5))),
                        )))
                    }
                    _ => {
                        // Glass
                        world.add(Arc::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dielectric::new(1.5)),
                        )))
                    }
                }
            }

            // Big spheres
            world.add(Arc::new(Sphere::new(
                Point::new(0.0, 1.0, 0.0),
                1.0,
                Arc::new(Dielectric::new(1.5)),
            )));
            world.add(Arc::new(Sphere::new(
                Point::new(-4.0, 1.0, 0.0),
                1.0,
                Arc::new(Lambertian::new_from_color(Color::new(0.4, 0.2, 0.1))),
            )));
            world.add(Arc::new(Sphere::new(
                Point::new(4.0, 1.0, 0.0),
                1.0,
                Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
            )));
        }
    }

    let camera = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    Scene {
        objects: world,
        camera,
    }
}

pub fn two_spheres() -> Scene {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::new_from_colors(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker)),
    )));

    let camera = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0,
        0.0,
        1.0,
    );

    Scene {
        objects: world,
        camera,
    }
}
