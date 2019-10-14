use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::pixel::Pixel;

pub struct HitRec {
    pub hit: bool,
    pub normal: Vec3,
    pub t_value: f32,
    pub color: Pixel,
}

pub trait Hittable {
    fn hit(&self, r: &Ray) -> HitRec;
}
