use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    Point,
};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.magnitude_squared() - self.radius.powi(2);

        // Check the determinant of the intersection quadratic implies real solutions
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root in the hit range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        Some(HitRecord::new(
            ray.at(root),
            root,
            (ray.at(root) - self.center) / self.radius,
            ray,
        ))
    }
}
