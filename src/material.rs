use crate::{hittable::HitRecord, math::Color, ray::Ray};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, attenuation: &Color) -> Option<HitRecord>;
}
