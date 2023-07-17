use crate::{
    math::{Color, Point},
    perlin::{Perlin, DEFAULT_TURBULENCE_DEPTH},
};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
}

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _: f64, _: f64, _: Point) -> Color {
        self.color
    }
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Self {
        Self { odd, even }
    }

    pub fn new_from_colors(odd: Color, even: Color) -> Self {
        Self::new(
            Box::new(SolidColorTexture::new(odd)),
            Box::new(SolidColorTexture::new(even)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f64, _: f64, p: Point) -> Color {
        // Use the turbulent noise to perturb a sine wave, & use that to modify the color.
        // Gives a marlbed wave effect.
        // Lots of scope for material generation here if we expose some of the noise settings to the caller
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0
                + (self.scale * p.z + 10.0 * self.noise.turbulence(p, DEFAULT_TURBULENCE_DEPTH))
                    .sin())
    }
}
