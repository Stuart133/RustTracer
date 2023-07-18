#![allow(dead_code)]
use std::{path::Path, sync::Arc};

use crate::{
    camera::{self, Camera},
    hittable::HittableList,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    math::{random_color, random_range, Color, Point, Vector},
    objects::{MovingSphere, Sphere},
    rectangle::{Cuboid, XyRectangle, XzRectangle, YzRectangle},
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColorTexture},
    ASPECT_RATIO,
};

pub struct Scene {
    pub objects: HittableList,
    pub background: Color,
    pub camera: Camera,
    pub image: Image,
}

pub struct Image {
    pub width: i64,
    pub height: i64,
    pub samples_per_pixel: u64,
}

impl Image {
    pub fn new(width: i64, samples_per_pixel: u64, aspect_ratio: f64) -> Self {
        Self {
            width,
            height: (width as f64 / aspect_ratio) as i64,
            samples_per_pixel,
        }
    }
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
                    x if x < 0.2 => {
                        // Diffuse checker
                        let checker = CheckerTexture::new(
                            Box::new(SolidColorTexture::new(random_color(0.0, 1.0))),
                            Box::new(SolidColorTexture::new(random_color(0.0, 1.0))),
                        );

                        world.add(Arc::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Lambertian::new(Arc::new(checker))),
                        )));
                    }
                    x if x < 0.3 => {
                        // Noise texture
                        world.add(Arc::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(10.0)))),
                        )));
                    }
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
            world.add(Arc::new(Sphere::new(
                Point::new(-8.0, 1.0, 0.0),
                1.0,
                Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(4.0)))),
            )))
        }
    }

    let camera = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.0,
        10.0,
        0.0,
        0.0,
    );

    let image = Image::new(1440, 500, ASPECT_RATIO);

    Scene {
        objects: world,
        camera,
        background: Color::new(0.7, 0.8, 1.00),
        image,
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

    let image = Image::new(400, 100, ASPECT_RATIO);

    Scene {
        objects: world,
        camera,
        background: Color::new(0.7, 0.8, 1.00),
        image,
    }
}

pub fn two_perlin_spheres() -> Scene {
    let mut world = HittableList::new();

    let perlin = Arc::new(NoiseTexture::new(4.0));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin)),
    )));

    let camera = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let image = Image::new(400, 100, ASPECT_RATIO);

    Scene {
        objects: world,
        camera,
        background: Color::new(0.7, 0.8, 1.00),
        image,
    }
}

pub fn earth() -> Scene {
    let mut world = HittableList::new();

    let earth_texture = Arc::new(ImageTexture::new(Path::new("./textures/earthmap.jpg")).unwrap());
    let surface = Arc::new(Lambertian::new(earth_texture));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        2.0,
        surface,
    )));

    let camera = Camera::new(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let image = Image::new(400, 100, ASPECT_RATIO);

    Scene {
        objects: world,
        camera,
        background: Color::new(0.7, 0.8, 1.00),
        image,
    }
}

pub fn lights() -> Scene {
    let mut world = HittableList::new();

    let noise = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(noise.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(noise.clone())),
    )));

    let light = Arc::new(DiffuseLight::new_from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 8.0, 0.0),
        2.0,
        light.clone(),
    )));
    world.add(Arc::new(XyRectangle::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        light.clone(),
    )));

    let camera = Camera::new(
        Point::new(26.0, 3.0, 6.0),
        Point::new(0.0, 2.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    let image = Image::new(400, 400, ASPECT_RATIO);

    Scene {
        objects: world,
        background: Color::new(0.0, 0.0, 0.0),
        camera,
        image,
    }
}

pub fn cornell_box() -> Scene {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.12)));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YzRectangle::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRectangle::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XzRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XzRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XyRectangle::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    objects.add(Arc::new(Cuboid::new(
        Point::new(130.0, 0.0, 65.0),
        Point::new(295.0, 165.0, 230.0),
        white.clone(),
    )));
    objects.add(Arc::new(Cuboid::new(
        Point::new(265.0, 0.0, 295.0),
        Point::new(430.0, 330.0, 460.0),
        white.clone(),
    )));

    let image = Image::new(600, 500, 1.0);

    let camera = Camera::new(
        Point::new(278.0, 278.0, -800.0),
        Point::new(278.0, 278.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        40.0,
        1.0,
        0.0,
        10.0,
        0.0,
        0.0,
    );

    Scene {
        objects,
        background: Color::new(0.0, 0.0, 0.0),
        camera,
        image,
    }
}
