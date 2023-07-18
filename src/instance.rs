use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    math::{Point, Rotation, Vector},
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

pub struct Rotate {
    hittable: Box<dyn Hittable>,
    rotation: Rotation,
    aabb: Option<AABB>,
}

impl Rotate {
    // TODO: Support more than just y rotation
    pub fn new(hittable: Box<dyn Hittable>, gamma: f64) -> Self {
        let rads = gamma.to_radians();
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
                    rotation: Rotation::from_euler_angles(0.0, rads, 0.0),
                    aabb: Some(AABB::new(min, max)),
                }
            }
            None => Self {
                hittable,
                rotation: Rotation::from_euler_angles(0.0, rads, 0.0),
                aabb: None,
            },
        }
    }
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Rotate the ray opposite to the transform
        let rotated_ray = Ray::new(
            self.rotation.inverse_transform_point(&ray.origin()),
            self.rotation.inverse_transform_vector(&ray.direction()),
            ray.time(),
        );

        match self.hittable.hit(&rotated_ray, t_min, t_max) {
            Some(mut hit) => {
                // Rotate the hit in the direction of the transform
                hit.p = self.rotation.transform_point(&hit.p);
                hit.normal = self.rotation.transform_vector(&hit.normal);

                Some(hit)
            }
            None => None,
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        self.aabb.clone()
    }
}
