use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, r: &Ray) -> f32 {
        let oc = r.origin - self.center;
        let a = r.direction.len();
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.len() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            -1.0
        } else {
            (-b - discriminant.sqrt()) / (2.0*a)
        }
    }
}