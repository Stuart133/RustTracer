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
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;

        self.ran[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
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
}
