use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable, HittableList},
    material::Material,
    math::{Point, Vector},
    ray::Ray,
};

pub struct Cuboid {
    min: Point,
    max: Point,
    sides: HittableList,
}

impl Cuboid {
    pub fn new(p0: Point, p1: Point, material: Arc<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XyRectangle::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            material.clone(),
        )));
        sides.add(Arc::new(XyRectangle::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            material.clone(),
        )));

        sides.add(Arc::new(XzRectangle::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            material.clone(),
        )));
        sides.add(Arc::new(XzRectangle::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            material.clone(),
        )));

        sides.add(Arc::new(YzRectangle::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p0.x,
            material.clone(),
        )));
        sides.add(Arc::new(YzRectangle::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            material.clone(),
        )));

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}

pub struct XyRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl XyRectangle {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XyRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Calculate hit position of the xy-plane along the ray
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x + t * ray.direction().x;
        let y = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
            Vector::new(0.0, 0.0, 1.0),
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        // We need to pad the z-axis of the bounding box otherwise it will be infinitely thin
        Some(AABB::new(
            Point::new(self.x0, self.y0, self.k - 0.0001),
            Point::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

pub struct XzRectangle {
    material: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl XzRectangle {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XzRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Calculate hit position of the xy-plane along the ray
        let t = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
            Vector::new(0.0, 1.0, 0.0),
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        // We need to pad the z-axis of the bounding box otherwise it will be infinitely thin
        Some(AABB::new(
            Point::new(self.x0, self.k - 0.0001, self.z0),
            Point::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

pub struct YzRectangle {
    material: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl YzRectangle {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Arc<dyn Material>) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YzRectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Calculate hit position of the xy-plane along the ray
        let t = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y + t * ray.direction().y;
        let z = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
            Vector::new(1.0, 0.0, 0.0),
            ray,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        // We need to pad the z-axis of the bounding box otherwise it will be infinitely thin
        Some(AABB::new(
            Point::new(self.k - 0.0001, self.y0, self.z0),
            Point::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
