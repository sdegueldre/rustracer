#[derive(Copy, Clone)]
pub struct Pixel {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Pixel {
    pub fn new(r: f32, g: f32, b: f32) -> Pixel {
        Pixel {r: r, g: g, b: b}
    }

    pub fn ppm(&self) -> String {
        format!("{} {} {}", (self.r.sqrt()*255.0).floor(), (self.g.sqrt()*255.0).floor(), (self.b.sqrt()*255.0).floor())
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.ppm())
    }
}

use std::ops::*;
impl Mul<Pixel> for f32 {
    type Output = Pixel;
    fn mul(self, col: Pixel) -> Pixel {
        Pixel::new(self * col.r, self * col.g, self * col.b)
    }
}
