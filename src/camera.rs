use nalgebra::Unit;

use crate::{math::Vector, ray::Ray, Point};

pub struct Camera {
    // lookfrom: Point,
    // lookat: Point,
    // view_up: Vector,
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        view_up: Vector,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // Create an orthonormal basis for the camera coordinate system
        let w = Unit::new_normalize(lookfrom - lookat);
        let u = Unit::new_normalize(view_up.cross(&w));
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = viewport_width * *u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - *w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
