use crate::{
    hittable::{Hittable, HittableList},
    math::{Color, Point, Vector},
    MIN_INTERSECTION_DISTANCE,
};

#[derive(Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
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

    #[inline]
    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn color(&self, background: Color, world: &HittableList, depth: i64) -> Color {
        // If we hit depth, the ray doesn't gather any further light
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(self, MIN_INTERSECTION_DISTANCE, f64::MAX) {
            Some(hit) => {
                let emitted = hit.material.emitted(hit.u, hit.v, hit.p);

                match hit.material.scatter(self, &hit) {
                    Some(scatter) => {
                        emitted
                            + scatter.attentuation.component_mul(&scatter.ray.color(
                                background,
                                world,
                                depth - 1,
                            ))
                    }
                    None => emitted,
                }

                // TODO: Move these to their own diffuse materials
                // A diffuse scatter that produces a tigher scatter (Proportional to cos(phi)^3)
                //let target = hit.p + hit.normal + random_in_unit_sphere();

                // An different diffuse scattering method that is not distributed in proportion to the angle with the normal
                // let target = hit.p + random_in_hemisphere(hit.normal);
            }
            None => background,
        }
    }
}
