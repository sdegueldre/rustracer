use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::hittable::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> HitRec {
        let oc = r.origin - self.center;
        let a = r.direction.len();
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.len() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            HitRec { hit: false, normal: Vec3::new(0.0, 0.0, 0.0) }
        } else {
            let t = (-b - discriminant.sqrt()) / (2.0*a);
            HitRec {
                hit: true,
                normal: (r.point_at(t) - self.center).normalize(),
            }
        }
    }
}