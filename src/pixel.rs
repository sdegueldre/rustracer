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
        format!("{} {} {}", (self.r*255.0).floor(), (self.g*255.0).floor(), (self.b*255.0).floor())
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.ppm())
    }
}