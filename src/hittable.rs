use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRec {
    pub hit: bool,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray) -> HitRec;
}