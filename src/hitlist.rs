use crate::hittable::*;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct HitList {
    pub items: Vec<Box<dyn Hittable>>,
}

impl Hittable for HitList {
    fn hit(&self, r: &Ray) -> HitRec {
        let mut closest_hit = HitRec { hit: false, normal: Vec3::new(0.0, 0.0, 0.0), t_value: std::f32::MAX };
        for item in &self.items {
            let hit = item.hit(&r);
            if hit.hit && hit.t_value > 0.0 && hit.t_value < closest_hit.t_value {
                closest_hit = hit;
            }
        }
        closest_hit
    }
}
