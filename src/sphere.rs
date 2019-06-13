use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::geometry::Vec3;
use crate::material::Material;
use std::sync::Arc;

pub(crate) struct Sphere {
    pub(crate) center: Vec3,
    pub(crate) radius: f32,
    pub(crate) material: Arc<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(ray.direction, ray.direction);
        let b = Vec3::dot(oc, ray.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t = (-b - (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }

            let t = (-b + (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_parameter(t);
                return Some(HitRecord {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}
