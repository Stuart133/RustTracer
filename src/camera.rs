use nalgebra::Unit;

use crate::{
    math::{random_in_unit_disk, Vector},
    ray::Ray,
    Point,
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vector,
    vertical: Vector,
    u: Vector,
    v: Vector,
    w: Vector,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        view_up: Vector,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // Create an orthonormal basis for the camera coordinate system
        let w = *Unit::new_normalize(lookfrom - lookat);
        let u = *Unit::new_normalize(view_up.cross(&w));
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
