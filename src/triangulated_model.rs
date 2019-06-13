use crate::hitable::{Hitable, HitRecord};
use crate::ray::Ray;
use crate::geometry::Vec3;
use crate::material::Material;
use crate::mesh::Mesh;
use std::sync::Arc;

pub(crate) struct TriangulatedModel {
    pub(crate) mesh: Mesh,
    pub(crate) material: Arc<dyn Material>,
}

impl TriangulatedModel {
    pub(crate) fn new(mesh: Mesh, material: Arc<dyn Material>) -> TriangulatedModel {
        Self {
            mesh,
            material,
        }
    }
}

impl Hitable for TriangulatedModel {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        for (v0, v1, v2) in self.mesh.iter_triangles() {
            if let Some(t) = ray_triangle_intersect(ray, v0, v1, v2) {
                if !(t > t_min && t < t_max) {
                    continue;
                }

                return Some(HitRecord {
                    t,
                    p: ray.point_at_parameter(t),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}


fn ray_triangle_intersect(ray: &Ray, v0: Vec3, v1: Vec3, v2: Vec3) -> Option<f32> {
    // Algorithm based on
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution

    // compute plane's normal
    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;
    let n = Vec3::cross(v0v1, v0v2);

    // Step 1: find p
    // check if ray and plane are parallel?
    let n_dot_ray_direction = Vec3::dot(n, ray.direction);
    if n_dot_ray_direction.abs() < std::f32::EPSILON {
        return None; // parallel so they do not intersect
    }

    // compute d parameter using equation 2
    let d = Vec3::dot(n, v0);

    // compute t (eq. 3)
    let t = (Vec3::dot(n, ray.origin) + d) / n_dot_ray_direction;
    // check if the triangle is in behind the ray
    if t < 0.0 {
        return None;
    }

    // compute the intersection point using eq. 1
    let p = ray.origin + t * ray.direction;

    // Step 2: inside-out test
    // c - vector perpendicular to triangle's plane

    // edge 0
    let edge0 = v1 - v0;
    let vp0 = p - v0;
    let c = Vec3::cross(edge0, vp0);
    if Vec3::dot(n, c) < 0.0 {
        return None; // p is on the right side
    }

    // edge 1
    let edge1 = v2 - v1;
    let vp1 = p - v1;
    let c = Vec3::cross(edge1, vp1);
    if Vec3::dot(n, c) < 0.0 {
        return None; // p is on the right side
    }

    // edge 2
    let edge2 = v0 - v2;
    let vp2 = p - v2;
    let c = Vec3::cross(edge2, vp2);
    if Vec3::dot(n, c) < 0.0 {
        return None; // p is on the right side
    }

    Some(t)
}
