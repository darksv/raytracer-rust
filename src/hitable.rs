use crate::ray::Ray;
use crate::geometry::Vec3;
use crate::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct HitRecord {
    pub(crate) t: f32,
    pub(crate) point: Vec3,
    pub(crate) normal: Vec3,
    pub(crate) material: Arc<dyn Material>,
}

pub(crate) trait Hitable: Send+Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}