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

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);

        if sphere.hit(self) {
            Color::new(1.0, 0.0, 0.0)
        } else {
            // The ray hit nothing, render the background gradient
            let t = 0.5 * (Unit::new_normalize(self.direction).y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
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

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius.powi(2);

        // Check the determinant of the intersection quadratic implies real solutions
        return b.powi(2) - 4.0 * a * c > 0.0;
    }
}
