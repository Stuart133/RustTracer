use nalgebra::{Point3, Vector3};

pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Point = Point3<f64>;

#[inline]
pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

#[inline]
pub fn random_vector() -> Vector {
    Vector::new(rand::random(), rand::random(), rand::random())
}

#[inline]
pub fn random_vector_range(min: f64, max: f64) -> Vector {
    Vector::new(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vector {
    loop {
        let p = random_vector_range(-1.0, 1.0);
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}