use nalgebra::Unit;

use crate::{
    hittable::{Hittable, Sphere},
    Color, Point, Vector,
};

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

    #[inline]
    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);

        match sphere.hit(self, -100.0, 100.0) {
            Some(hit) => {
                let normal = Unit::new_normalize(self.at(hit.t) - sphere.center());
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
