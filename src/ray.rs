use nalgebra::Unit;

use crate::{
    hittable::{Hittable, HittableList},
    math::{random_in_unit_sphere, random_unit_vector, Color, Point, Vector},
    MIN_INTERSECTION_DISTANCE,
};

#[derive(Debug)]
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

    pub fn color(&self, world: &HittableList, depth: i64) -> Color {
        // If we hit depth, the ray doesn't contribute any light
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(self, MIN_INTERSECTION_DISTANCE, f64::MAX) {
            Some(hit) => {
                let target = hit.p + hit.normal + random_unit_vector();
                // 0.5 * (hit.normal + Color::new(1.0, 1.0, 1.0))
                0.5 * Ray::new(hit.p, target - hit.p).color(world, depth - 1)
            }
            None => {
                let t = 0.5 * (Unit::new_normalize(self.direction).y + 1.0);
                (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
