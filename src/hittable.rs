use std::sync::Arc;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    math::{random_color, random_range, Color, Vector},
    objects::{MovingSphere, Sphere},
    ray::Ray,
    Point,
};

pub trait Hittable: Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn random_scene() -> Self {
        let mut world = HittableList::new();

        let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
        world.add(Box::new(Sphere::new(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            ground_material,
        )));

        for a in -11..11 {
            for b in -11..11 {
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
                            world.add(Box::new(MovingSphere::new(
                                center,
                                end_center,
                                0.0,
                                1.0,
                                0.2,
                                Arc::new(Lambertian::new(random_color(0.0, 1.0))),
                            )))
                        }
                        x if x < 0.95 => {
                            // Metal
                            world.add(Box::new(Sphere::new(
                                center,
                                0.2,
                                Arc::new(Metal::new(
                                    random_color(0.5, 1.0),
                                    random_range(0.0, 0.5),
                                )),
                            )))
                        }
                        _ => {
                            // Glass
                            world.add(Box::new(Sphere::new(
                                center,
                                0.2,
                                Arc::new(Dielectric::new(1.5)),
                            )))
                        }
                    }
                }

                // Big spheres
                world.add(Box::new(Sphere::new(
                    Point::new(0.0, 1.0, 0.0),
                    1.0,
                    Arc::new(Dielectric::new(1.5)),
                )));
                world.add(Box::new(Sphere::new(
                    Point::new(-4.0, 1.0, 0.0),
                    1.0,
                    Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
                )));
                world.add(Box::new(Sphere::new(
                    Point::new(4.0, 1.0, 0.0),
                    1.0,
                    Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
                )));
            }
        }

        world
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}

pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub normal: Vector,
    pub material: Arc<dyn Material>,
    pub face: Face,
}

impl HitRecord {
    pub fn new(
        p: Point,
        t: f64,
        outward_normal: Vector,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        if ray.direction().dot(&outward_normal) > 0.0 {
            HitRecord {
                p,
                t,
                normal: -outward_normal,
                material,
                face: Face::Back,
            }
        } else {
            HitRecord {
                p,
                t,
                normal: outward_normal,
                material,
                face: Face::Front,
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Face {
    Front,
    Back,
}
