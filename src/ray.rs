use nalgebra::Unit;

use crate::{Color, Point, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    #[inline]
    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    #[inline]
    pub fn direction(&self) -> &Vector {
        &self.direction
    }

    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);

        match sphere.hit(self) {
            Some(t) => {
                let normal = Unit::new_normalize(self.at(t) - sphere.center);
                0.5 * Color::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0)
            }
            None => {
                // The ray hit nothing, render the background gradient
                let t = 0.5 * (Unit::new_normalize(self.direction).y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.magnitude_squared() - self.radius.powi(2);

        // Check the determinant of the intersection quadratic implies real solutions
        let discriminant = half_b.powi(2) - a * c;
        if discriminant > 0.0 {
            return Some((-half_b - discriminant.sqrt()) / a);
        } else {
            None
        }
    }
}
