use nalgebra::Unit;

use crate::{
    hittable::HitRecord,
    math::{near_zero, random_in_unit_sphere, random_unit_vector, reflect, Color},
    ray::Ray,
};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
}

pub struct ScatterRecord {
    pub ray: Ray,
    pub attentuation: Color,
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    // A diffuse scatter that produces a lambertian distribution (Proportional to cos(phi))
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit.normal
        }

        Some(ScatterRecord {
            ray: Ray::new(hit.p, scatter_direction),
            attentuation: self.albedo,
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&Unit::new_normalize(*ray_in.direction()), &hit.normal);

        // TODO: Absorb rays which scatter inside the original object
        Some(ScatterRecord {
            ray: Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere()),
            attentuation: self.albedo,
        })
    }
}
