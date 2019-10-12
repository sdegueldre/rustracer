use rand::distributions::{Normal, Distribution};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn random_unit_vector() -> Vec3 {
    let normal = Normal::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    Vec3::new(
        normal.sample(&mut rng) as f32,
        normal.sample(&mut rng) as f32,
        normal.sample(&mut rng) as f32
    ).normalize()
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x:x, y:y, z:z}
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn len(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.len();
        Vec3::new(self.x/len, self.y/len, self.z/len)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

// Operators

use std::ops::*;
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}
