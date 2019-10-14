use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::pixel::Pixel;
use crate::hittable::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Pixel,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> HitRec {
        let oc = r.origin - self.center;
        let a = 1.0;
        let b = 2.0 * oc.dot(&r.direction);
        let c = oc.len()*oc.len() - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;

        if discriminant < 0.0 {
            HitRec { hit: false, normal: Vec3::new(0.0, 0.0, 0.0), t_value: 0.0, color: self.color }
        } else {
            let t = (-b - discriminant.sqrt()) / 2.0;
            HitRec {
                hit: true,
                normal: (r.point_at(t) - self.center).normalize(),
                t_value: t,
                color: self.color,
            }
        }
    }
}
