use nalgebra::Unit;

use crate::{
    hittable::{Face, HitRecord},
    math::{near_zero, random_in_unit_sphere, random_unit_vector, Color, Vector},
    ray::{self, Ray},
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = match hit.face {
            Face::Front => 1.0 / self.refraction_index,
            Face::Back => self.refraction_index,
        };

        let unit_direction = Unit::new_normalize(*ray_in.direction());
        let refracted = refract(&unit_direction, &hit.normal, refraction_ratio);

        Some(ScatterRecord {
            ray: Ray::new(hit.p, refracted),
            attentuation: Color::new(1.0, 1.0, 1.0),
        })
    }
}

#[inline]
pub fn reflect(vector: &Vector, normal: &Vector) -> Vector {
    vector - 2.0 * vector.dot(normal) * normal
}

pub fn refract(uv: &Vector, normal: &Vector, etai_over_etat: f64) -> Vector {
    let cos_theta = (-uv).dot(normal).min(1.0);
    let r_out_perpendicular = etai_over_etat * (uv + cos_theta * normal);
    let r_out_parallel = -(1.0 - r_out_perpendicular.magnitude_squared()).abs().sqrt() * normal;

    r_out_parallel + r_out_perpendicular
}
