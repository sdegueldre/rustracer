use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(&self, r: &Ray) -> bool {
        let oc = r.origin - self.center;
        
        let a = r.direction.len();
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.len() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        discriminant > 0.0
    }
}