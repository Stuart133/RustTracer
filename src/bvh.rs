use std::{cmp::Ordering, sync::Arc};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::{
    aabb::AABB,
    hittable::{Hittable, HittableList},
};

pub struct BVHNode {
    aabb: AABB,

    // TODO: Figure out how to consume the hittable list without the need for ARCs
    // They probably aren't needed but right now we're using them as a crutch while we process
    // the hit list into a BVH tree
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(list: HittableList, start_time: f64, end_time: f64) -> Self {
        BVHNode::new_inner(&mut list.as_raw(), start_time, end_time)
    }

    fn new_inner(objects: &mut [Arc<dyn Hittable>], start_time: f64, end_time: f64) -> Self {
        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => match box_compare(&objects[0], &objects[1], rand::random()) {
                Ordering::Less => (objects[0].clone(), objects[1].clone()),
                _ => (objects[1].clone(), objects[0].clone()),
            },
            _ => {
                objects.sort_by(|a, b| box_compare(a, b, rand::random()));
                let mid = objects.len() / 2;

                let (l, r) = objects.split_at_mut(mid);
                (
                    Arc::new(BVHNode::new_inner(l, start_time, end_time)) as Arc<dyn Hittable>,
                    Arc::new(BVHNode::new_inner(r, start_time, end_time)) as Arc<dyn Hittable>,
                )
            }
        };

        let box_left = left.bounding_box(start_time, end_time);
        let box_right = right.bounding_box(start_time, end_time);
        if box_left.is_none() || box_right.is_none() {
            eprintln!("No bounding box in BVH constructor");
        }

        Self {
            aabb: AABB::surrounding_box(&box_left.unwrap(), &box_right.unwrap()),
            left,
            right,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
    ) -> Option<crate::hittable::HitRecord> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        match self.left.hit(ray, t_min, t_max) {
            Some(left_hit) => match self.right.hit(ray, t_min, left_hit.t) {
                Some(right_hit) => Some(right_hit),
                None => Some(left_hit),
            },
            None => self.right.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, _: f64, _: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match rng.gen_range(0..=2) {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => unreachable!(),
        }
    }
}

impl Into<usize> for Axis {
    fn into(self) -> usize {
        match self {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: Axis) -> Ordering {
    let box_a = a.bounding_box(0.0, 0.0);
    let box_b = b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        eprintln!("No bounding box in BVH constructor");
    }

    box_a.unwrap().minimum()[axis.into()].total_cmp(&box_b.unwrap().minimum()[axis.into()])
}
