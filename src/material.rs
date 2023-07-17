use std::sync::Arc;

use nalgebra::Unit;

use crate::{
    hittable::{Face, HitRecord},
    math::{near_zero, random_in_unit_sphere, random_unit_vector, Color, Point, Vector},
    ray::Ray,
    texture::{SolidColorTexture, Texture},
};

pub trait Material: Sync + Send {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord>;
    fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct ScatterRecord {
    pub ray: Ray,
    pub attentuation: Color,
}

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn new_from_color(albedo: Color) -> Self {
        Self::new(Arc::new(SolidColorTexture::new(albedo)))
    }
}

impl Material for Lambertian {
    // A diffuse scatter that produces a lambertian distribution (Proportional to cos(phi))
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if near_zero(&scatter_direction) {
            scatter_direction = hit.normal
        }

        Some(ScatterRecord {
            ray: Ray::new(hit.p, scatter_direction, ray_in.time()),
            attentuation: self.albedo.value(hit.u, hit.v, hit.p),
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
            ray: Ray::new(
                hit.p,
                reflected + self.fuzz * random_in_unit_sphere(),
                ray_in.time(),
            ),
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

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let refraction_ratio = match hit.face {
            Face::Front => 1.0 / self.refraction_index,
            Face::Back => self.refraction_index,
        };

        let unit_direction = Unit::new_normalize(*ray_in.direction());
        let cos_theta = -unit_direction.dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            ray: Ray::new(hit.p, direction, ray_in.time()),
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

pub struct DiffuseLight {
    emit: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Box<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn new_from_color(color: Color) -> Self {
        Self::new(Box::new(SolidColorTexture::new(color)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: Point) -> Color {
        self.emit.value(u, v, p)
    }
}
