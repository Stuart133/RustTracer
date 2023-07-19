use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::{Isotropic, Material},
    math::{Color, Vector},
    texture::Texture,
    MIN_INTERSECTION_DISTANCE,
};

pub struct ConstantVolume {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    negative_inverse_density: f64,
}

impl ConstantVolume {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: Box<dyn Texture>) -> Self {
        Self {
            boundary,
            negative_inverse_density: (-1.0 / density),
            phase_function: Arc::new(Isotropic::new(albedo)),
        }
    }

    pub fn new_from_color(boundary: Arc<dyn Hittable>, density: f64, color: Color) -> Self {
        Self {
            boundary,
            negative_inverse_density: (-1.0 / density),
            phase_function: Arc::new(Isotropic::new_from_color(color)),
        }
    }
}

impl Hittable for ConstantVolume {
    // This assumes the volume is convex
    // TODO: Investigate how to support concave volumes
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        let mut hit1 = match self.boundary.hit(ray, f64::NEG_INFINITY, f64::INFINITY) {
            Some(hit) => hit,
            None => return None,
        };
        let mut hit2 =
            match self
                .boundary
                .hit(ray, hit1.t + MIN_INTERSECTION_DISTANCE, f64::INFINITY)
            {
                Some(hit) => hit,
                None => return None,
            };

        hit1.t = hit1.t.max(t_min);
        hit2.t = hit2.t.min(t_max);

        if hit1.t >= hit2.t {
            return None;
        }

        hit1.t = hit1.t.max(0.0);

        let ray_length = ray.direction().magnitude();
        let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
        let hit_distance =
            self.negative_inverse_density * rand::random::<f64>().log(std::f64::consts::E);

        if hit_distance > distance_inside_boundary {
            return None;
        }

        // u, v & the normal vector are meaningless for this hit
        Some(HitRecord::new(
            hit1.t + (hit_distance / ray_length),
            0.0,
            0.0,
            Vector::new(1.0, 0.0, 0.0),
            ray,
            self.phase_function.clone(),
        ))
    }

    fn bounding_box(&self, start_time: f64, end_time: f64) -> Option<crate::aabb::AABB> {
        self.boundary.bounding_box(start_time, end_time)
    }
}
