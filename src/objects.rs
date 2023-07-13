use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    material::Material,
    math::Vector,
    ray::Ray,
    Point,
};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
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
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<crate::aabb::AABB> {
        Some(AABB::new(
            self.center - Vector::new(self.radius, self.radius, self.radius),
            self.center + Vector::new(self.radius, self.radius, self.radius),
        ))
    }
}

pub struct MovingSphere {
    start_center: Point,
    end_center: Point,
    start_time: f64,
    end_time: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        start_center: Point,
        end_center: Point,
        start_time: f64,
        end_time: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            start_center,
            end_center,
            start_time,
            end_time,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Point {
        self.start_center
            + ((time - self.start_time) / (self.end_time - self.start_time))
                * (self.end_center - self.start_center)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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
            (ray.at(root) - self.center(ray.time())) / self.radius,
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<AABB> {
        let start_box = AABB::new(
            self.center(start_time) - Vector::new(self.radius, self.radius, self.radius),
            self.center(start_time) + Vector::new(self.radius, self.radius, self.radius),
        );
        let end_box = AABB::new(
            self.center(end_time) - Vector::new(self.radius, self.radius, self.radius),
            self.center(end_time) + Vector::new(self.radius, self.radius, self.radius),
        );

        Some(AABB::surrounding_box(&start_box, &end_box))
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        hittable::{Face, Hittable},
        material::Lambertian,
        math::{Color, Point, Vector},
        ray::Ray,
        MIN_INTERSECTION_DISTANCE,
    };

    use super::Sphere;

    #[test]
    pub fn outward_ray_hit() {
        let sphere = Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Lambertian::new(Color::new(1.0, 1.0, 1.0))),
        );

        // This looks a bit random, but was a ray causing trouble on reflection due to 0 point intersection
        let ray = Ray::new(
            Point::new(0.0, 0.0, 0.0),
            Vector::new(-0.07757490284849644, 0.5715330690568323, -1.0),
            0.0,
        );

        let hit = sphere.hit(&ray, 0.0, f64::MAX).unwrap();
        let second_hit = sphere.hit(
            &Ray::new(hit.p, hit.normal, 0.0),
            MIN_INTERSECTION_DISTANCE,
            f64::MAX,
        );

        assert_eq!(Face::Front, hit.face);
        assert!(second_hit.is_none());
    }
}
