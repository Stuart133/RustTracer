use nalgebra::Unit;

use crate::{Color, Point, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let t = 0.5 * (Unit::new_normalize(self.direction).y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
