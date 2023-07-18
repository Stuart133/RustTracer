use crate::{math::Point, ray::Ray};

#[derive(Debug, Clone)]
pub struct AABB {
    minimum: Point,
    maximum: Point,
}

impl AABB {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Self { minimum, maximum }
    }

    pub fn surrounding_box(box0: &AABB, box1: &AABB) -> Self {
        let small = Point::new(
            box0.minimum.x.min(box1.minimum.x),
            box0.minimum.y.min(box1.minimum.y),
            box0.minimum.z.min(box1.minimum.z),
        );
        let big = Point::new(
            box0.maximum.x.max(box1.maximum.x),
            box0.maximum.y.max(box1.maximum.y),
            box0.maximum.z.max(box1.maximum.z),
        );

        AABB::new(small, big)
    }

    #[inline]
    pub fn minimum(&self) -> Point {
        self.minimum
    }

    #[inline]
    pub fn maximum(&self) -> Point {
        self.maximum
    }

    // TODO: Implement the optimized routine suggested here: https://raytracing.github.io/books/RayTracingTheNextWeek.html#boundingvolumehierarchies/anoptimizedaabbhitmethod
    // It's gonna need tests to check convergance with this method
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut min = t_min;
        let mut max = t_max;

        for a in 0..3 {
            let t0 = ((self.minimum[a] - ray.origin()[a]) / ray.direction()[a])
                .min((self.maximum[a] - ray.origin()[a]) / ray.direction()[a]);
            let t1 = ((self.minimum[a] - ray.origin()[a]) / ray.direction()[a])
                .max((self.maximum[a] - ray.origin()[a]) / ray.direction()[a]);

            min = t0.max(min);
            max = t1.min(max);

            if max <= min {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        math::{Point, Vector},
        ray::Ray,
    };

    use super::AABB;

    #[test]
    pub fn hit_bounding_box() {
        let aabb = AABB::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let ray_in_box = Ray::new(Point::new(0.5, 0.5, 0.5), Vector::new(1.0, 1.0, 1.0), 0.0);
        let ray_out_box = Ray::new(Point::new(0.0, 0.0, -0.5), Vector::new(0.1, 0.1, 1.0), 0.0);
        let ray_flat = Ray::new(Point::new(0.5, 0.5, 0.5), Vector::new(0.0, 0.0, 1.0), 0.0);

        assert!(aabb.hit(&ray_in_box, f64::MIN, f64::MAX));
        assert!(aabb.hit(&ray_out_box, f64::MIN, f64::MAX));
        assert!(aabb.hit(&ray_flat, f64::MIN, f64::MAX));
    }

    #[test]
    pub fn miss_bounding_box() {
        let aabb = AABB::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));
        let ray_missing = Ray::new(Point::new(2.0, 2.0, 2.0), Vector::new(1.0, 0.01, 0.01), 0.0);
        let ray_flat = Ray::new(
            Point::new(100.0, 100.0, 100.0),
            Vector::new(1.0, 0.0, 0.0),
            0.0,
        );

        assert!(!aabb.hit(&ray_missing, f64::MIN, f64::MAX));
        assert!(!aabb.hit(&ray_flat, f64::MIN, f64::MAX));
    }
}
