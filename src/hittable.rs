use std::rc::Rc;

use crate::{math::Vector, ray::Ray, Point};

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn from_hittable(hittable: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![hittable],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, hittable: Rc<dyn Hittable>) {
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
    pub normal: Vector,
    pub t: f64,
    pub face: Face,
}

impl HitRecord {
    pub fn new(p: Point, outward_normal: Vector, t: f64, ray: &Ray) -> Self {
        if ray.direction().dot(&outward_normal) > 0.0 {
            HitRecord {
                p,
                normal: -outward_normal,
                t,
                face: Face::Front,
            }
        } else {
            HitRecord {
                p,
                normal: outward_normal,
                t,
                face: Face::Back,
            }
        }
    }
}

pub enum Face {
    Front,
    Back,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
