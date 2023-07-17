use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageError};

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

pub struct ImageTexture {
    image: DynamicImage,
}

impl ImageTexture {
    pub fn new(path: &Path) -> Result<Self, ImageError> {
        let image = image::open(path)?;

        Ok(Self { image })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _: Point) -> Color {
        // Clamp input coordinates to [0, 1] x [1, 0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip v to match image coordinates

        let mut i = (u * self.image.width() as f64) as u32;
        let mut j = (v * self.image.height() as f64) as u32;

        // Clamp integer mapping
        if i >= self.image.width() {
            i = self.image.width() - 1
        }
        if j >= self.image.height() {
            j = self.image.height() - 1
        }

        let color_scale = 1.0 / 255.0;
        let pixel = self.image.get_pixel(i, j);

        Color::new(
            color_scale * pixel.0[0] as f64,
            color_scale * pixel.0[1] as f64,
            color_scale * pixel.0[2] as f64,
        )
    }
}
