use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;

pub(crate) struct HitableList {
    hitables: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub(crate) fn from_vec(hitables: Vec<Box<dyn Hitable>>) -> Self {
        Self {
            hitables
        }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for hitable in &self.hitables {
            if let Some(record) = hitable.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t;
                hit_record = Some(record);
            }
        }

        hit_record
    }
}