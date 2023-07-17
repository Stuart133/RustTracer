use rand::Rng;

use crate::math::Point;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ran: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let ran = (0..POINT_COUNT).map(|_| rand::random()).collect();

        Self {
            ran,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }

    pub fn noise(&self, p: Point) -> f64 {
        // TODO: Figure how this all works
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[0.0; 2]; 2]; 2];

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

        Self::trilinear_interpolate(&c, u, v, w)
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

    fn trilinear_interpolate(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        (0..2).fold(0.0, |acc, i| {
            (0..2).fold(acc, |acc, j| {
                (0..2).fold(acc, |acc, k| {
                    acc + (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k]
                })
            })
        })
    }
}
