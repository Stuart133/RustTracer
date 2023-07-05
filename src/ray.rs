use nalgebra::Unit;

use crate::{
    hittable::{Hittable, HittableList},
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

    pub fn color(&self, world: &HittableList) -> Color {
        match world.hit(self, 0.0, f64::MAX) {
            Some(hit) => 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0)),
            None => {
                let t = 0.5 * (Unit::new_normalize(self.direction).y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
