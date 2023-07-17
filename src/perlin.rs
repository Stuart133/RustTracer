use rand::Rng;

use crate::math::{random_unit_vector, Point, Vector};

const POINT_COUNT: usize = 256;
pub const DEFAULT_TURBULENCE_DEPTH: usize = 7;

pub struct Perlin {
    ran: Vec<Vector>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let ran = (0..POINT_COUNT).map(|_| random_unit_vector()).collect();

        Self {
            ran,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn turbulence(&self, p: Point, depth: usize) -> f64 {
        let mut p = p;
        let mut weight = 1.0;

        (0..depth)
            .fold(0.0, |acc, _| {
                let accum = acc + weight * self.noise(p);
                weight *= 0.5;
                p *= 2.0;

                accum
            })
            .abs()
    }

    pub fn noise(&self, p: Point) -> f64 {
        // TODO: Figure how this all works
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vector::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.ran[self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]]
                }
            }
        }

        Self::perlin_interpolate(&c, u, v, w)
    }

    fn generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = (0..POINT_COUNT).collect();

        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut [usize]) {
        for i in (0..p.len() - 1).rev() {
            let target = rand::thread_rng().gen_range(0..i + 1);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    fn perlin_interpolate(c: &[[[Vector; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        // Use a hermite cubic to round off the smoothing
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        (0..2).fold(0.0, |acc, i| {
            (0..2).fold(acc, |acc, j| {
                (0..2).fold(acc, |acc, k| {
                    let weight = Vector::new(u - i as f64, v - j as f64, w - k as f64);
                    acc + (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(&weight)
                })
            })
        })
    }
}
