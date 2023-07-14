use std::sync::Arc;

use crate::{aabb::AABB, material::Material, math::Vector, ray::Ray, Point};

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB>;
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.objects.push(hittable)
    }

    pub fn as_raw(self) -> Vec<Arc<dyn Hittable>> {
        self.objects
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

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut outer_box = None;

        // TODO: Ask Merlin about mapping this properly
        for object in self.objects.iter() {
            match object.bounding_box(start_time, end_time) {
                Some(aabb) => match outer_box {
                    Some(inner_aabb) => {
                        outer_box = Some(AABB::surrounding_box(&aabb, &inner_aabb));
                    }
                    None => outer_box = Some(aabb),
                },
                None => return None,
            }
        }

        outer_box
    }
}

pub struct HitRecord {
    pub p: Point,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub normal: Vector,
    pub material: Arc<dyn Material>,
    pub face: Face,
}

impl HitRecord {
    pub fn new(
        p: Point,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vector,
        ray: &Ray,
        material: Arc<dyn Material>,
    ) -> Self {
        if ray.direction().dot(&outward_normal) > 0.0 {
            HitRecord {
                p,
                t,
                u,
                v,
                normal: -outward_normal,
                material,
                face: Face::Back,
            }
        } else {
            HitRecord {
                p,
                t,
                u,
                v,
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
