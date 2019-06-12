use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::geometry::Vec3;
use crate::random_in_unit_sphere;

pub(crate) struct ScatterInfo {
    pub(crate) attenuation: Vec3,
    pub(crate) scattered: Ray,
}

pub(crate) trait Material {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<ScatterInfo>;
}

pub(crate) struct Lambertian {
    pub(crate) albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<ScatterInfo> {
        let target = hr.p + hr.normal + random_in_unit_sphere();
        Some(ScatterInfo {
            attenuation: self.albedo,
            scattered: Ray::new(hr.p, target - hr.p),
        })
    }
}

pub(crate) struct Metal {
    pub(crate) albedo: Vec3,
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v, n) * n
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<ScatterInfo> {
        let reflected = reflect(r_in.direction.normalize(), hr.normal);
        if Vec3::dot(reflected, hr.normal) > 0.0 {
            Some(ScatterInfo {
                attenuation: self.albedo,
                scattered: Ray::new(hr.p, reflected),
            })
        } else {
            None
        }
    }
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalize();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n * dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

pub(crate) struct Dielectric {
    pub(crate) ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<ScatterInfo> {
        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(r_in.direction, hr.normal) > 0.0 {
            (
                -hr.normal,
                self.ref_idx,
                self.ref_idx * Vec3::dot(r_in.direction, hr.normal) / r_in.direction.len()
            )
        } else {
            (
                hr.normal,
                1.0 / self.ref_idx,
                -Vec3::dot(r_in.direction, hr.normal) / r_in.direction.len()
            )
        };

        let direction = match refract(r_in.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
                if rand::random::<f32>() < reflect_prob {
                    reflect(r_in.direction, hr.normal)
                } else {
                    refracted
                }
            }
            None => reflect(r_in.direction, hr.normal),
        };

        Some(ScatterInfo {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hr.p, direction),
        })
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}