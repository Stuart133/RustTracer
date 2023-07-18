// TODO: We can probably leverage the features of nalgebra to do the rotations/translations

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    math::{Point, Vector},
    ray::Ray,
};

pub struct Translate {
    hittable: Box<dyn Hittable>,
    offset: Vector,
}

impl Translate {
    pub fn new(hittable: Box<dyn Hittable>, offset: Vector) -> Self {
        Self { hittable, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        match self.hittable.hit(&offset_ray, t_min, t_max) {
            Some(mut hit) => {
                hit.p += self.offset;
                // hit.normal += self.offset;

                Some(hit)
            }
            None => None,
        }
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<crate::aabb::AABB> {
        match self.hittable.bounding_box(start_time, end_time) {
            Some(aabb) => Some(AABB::new(
                aabb.minimum() + self.offset,
                aabb.maximum() + self.offset,
            )),
            None => None,
        }
    }
}

pub struct RotateY {
    hittable: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    aabb: Option<AABB>,
}

impl RotateY {
    pub fn new(hittable: Box<dyn Hittable>, theta: f64) -> Self {
        let rads = theta.to_radians();
        let sin_theta = rads.sin();
        let cos_theta = rads.cos();

        // TODO: Probably should plumb time in here somehow
        let bound = hittable.bounding_box(0.0, 1.0);
        match bound {
            Some(bound) => {
                let mut min = Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
                let mut max = Point::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

                for i in 0..2 {
                    for j in 0..2 {
                        for k in 0..2 {
                            let x =
                                i as f64 * bound.maximum().x + (1.0 - i as f64) * bound.minimum().x;
                            let y =
                                j as f64 * bound.maximum().y + (1.0 - j as f64) * bound.minimum().y;
                            let z =
                                k as f64 * bound.maximum().z + (1.0 - k as f64) * bound.minimum().z;

                            let newx = cos_theta * x + sin_theta * z;
                            let newz = -sin_theta * x + cos_theta * z;

                            let tester = Vector::new(newx, y, newz);

                            for c in 0..3 {
                                min[c] = min[c].min(tester[c]);
                                max[c] = max[c].max(tester[c]);
                            }
                        }
                    }
                }

                Self {
                    hittable,
                    sin_theta,
                    cos_theta,
                    aabb: Some(AABB::new(min, max)),
                }
            }
            None => Self {
                hittable,
                sin_theta,
                cos_theta,
                aabb: None,
            },
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        // Transform the x/z coordinates of the origin & direction with the rotation
        origin[0] = self.cos_theta * ray.origin()[0] - self.sin_theta * ray.origin()[2];
        origin[2] = self.sin_theta * ray.origin()[0] + self.cos_theta * ray.origin()[2];

        direction[0] = self.cos_theta * ray.direction()[0] - self.sin_theta * ray.direction()[2];
        direction[2] = self.sin_theta * ray.direction()[0] + self.cos_theta * ray.direction()[2];

        let rotated_ray = Ray::new(origin, direction, ray.time());

        match self.hittable.hit(&rotated_ray, t_min, t_max) {
            Some(mut hit) => {
                // Transform the hit x/z coordinates and surface normal with the rotaion
                let mut p = hit.p;
                let mut normal = hit.normal;

                p[0] = self.cos_theta * hit.p[0] + self.sin_theta * hit.p[2];
                p[2] = -self.sin_theta * hit.p[0] + self.cos_theta * hit.p[2];

                normal[0] = self.cos_theta * hit.normal[0] + self.sin_theta * hit.normal[2];
                normal[2] = -self.sin_theta * hit.normal[0] + self.cos_theta * hit.normal[2];

                hit.p = p;
                hit.normal = normal;

                Some(hit)
            }
            None => None,
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        self.aabb.clone()
    }
}
