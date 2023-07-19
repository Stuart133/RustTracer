#![allow(dead_code)]
use std::{path::Path, sync::Arc};

use rand::Rng;

use crate::{
    bvh::BVHNode,
    camera::Camera,
    hittable::HittableList,
    instance::{Rotate, Translate},
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    math::{random_color, random_point, random_range, Color, Point, Vector},
    rectangle::{Cuboid, XyRectangle, XzRectangle, YzRectangle},
    sphere::{MovingSphere, Sphere},
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColorTexture},
    volumes::ConstantVolume,
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

pub fn week_scene(floor_n: i64, sphere_n: i64) -> Scene {
    let mut floor = HittableList::new();
    let ground = Arc::new(Lambertian::new_from_color(Color::new(0.48, 0.83, 0.53)));

    for i in 0..floor_n {
        for j in 0..floor_n {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rand::thread_rng().gen_range(1.0..101.0);
            let z1 = z0 + w;

            floor.add(Arc::new(Cuboid::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BVHNode::new(floor, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XzRectangle::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector::new(30.0, 0.0, 0.0);
    let moving_sphere = Arc::new(Lambertian::new_from_color(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere,
    )));

    objects.add(Arc::new(Sphere::new(
        Point::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantVolume::new_from_color(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantVolume::new_from_color(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth = Arc::new(Lambertian::new(Arc::new(
        ImageTexture::new(Path::new("./textures/earthmap.jpg")).unwrap(),
    )));
    objects.add(Arc::new(Sphere::new(
        Point::new(400.0, 200.0, 400.0),
        100.0,
        earth,
    )));
    let perlin = Arc::new(NoiseTexture::new(2.0));
    objects.add(Arc::new(Sphere::new(
        Point::new(200.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin)),
    )));

    let mut box_spheres = HittableList::new();
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    for _ in 0..sphere_n {
        box_spheres.add(Arc::new(Sphere::new(
            random_point(0.0..165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Box::new(Rotate::new(
            Box::new(BVHNode::new(box_spheres, 0.0, 1.0)),
            15.0,
        )),
        Vector::new(-100.0, 270.0, 395.0),
    )));

    let image = Image::new(800, 10_000, 1.0);

    let camera = Camera::new(
        Point::new(478.0, 278.0, -600.0),
        Point::new(278.0, 278.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        40.0,
        1.0,
        0.0,
        10.0,
        0.0,
        1.0,
    );

    Scene {
        objects,
        background: Color::new(0.0, 0.0, 0.0),
        camera,
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

    let box1 = Box::new(Cuboid::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Box::new(Rotate::new(box1, 15.0));
    let box1 = Translate::new(box1, Vector::new(265.0, 0.0, 295.0));
    objects.add(Arc::new(box1));

    let box2 = Box::new(Cuboid::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Box::new(Rotate::new(box2, -18.0));
    let box2 = Translate::new(box2, Vector::new(130.0, 0.0, 65.0));
    objects.add(Arc::new(box2));

    let image = Image::new(600, 200, 1.0);

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

pub fn cornell_box_smoke() -> Scene {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.12)));
    let light = Arc::new(DiffuseLight::new_from_color(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YzRectangle::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    )));
    objects.add(Arc::new(YzRectangle::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XzRectangle::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
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

    let box1 = Box::new(Cuboid::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let box1 = Box::new(Rotate::new(box1, 15.0));
    let box1 = Translate::new(box1, Vector::new(265.0, 0.0, 295.0));
    objects.add(Arc::new(ConstantVolume::new_from_color(
        Arc::new(box1),
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let box2 = Box::new(Cuboid::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let box2 = Box::new(Rotate::new(box2, -18.0));
    let box2 = Translate::new(box2, Vector::new(130.0, 0.0, 65.0));
    objects.add(Arc::new(ConstantVolume::new_from_color(
        Arc::new(box2),
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    let image = Image::new(600, 200, 1.0);

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
