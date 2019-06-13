use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::geometry::Vec3;
use crate::random_in_unit_sphere;

pub(crate) struct Scattered {
    pub(crate) attenuation: Vec3,
    pub(crate) scattered: Ray,
}

pub(crate) trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scattered>;
}

pub(crate) struct Lambertian {
    pub(crate) albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let target = hit_record.point + hit_record.normal + random_in_unit_sphere();
        Some(Scattered {
            attenuation: self.albedo,
            scattered: Ray::new(hit_record.point, target - hit_record.point),
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let reflected = reflect(ray.direction.normalize(), hit_record.normal);
        if Vec3::dot(reflected, hit_record.normal) > 0.0 {
            Some(Scattered {
                attenuation: self.albedo,
                scattered: Ray::new(hit_record.point, reflected),
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
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scattered> {
        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(ray.direction, hit_record.normal) > 0.0 {
            (
                -hit_record.normal,
                self.ref_idx,
                self.ref_idx * Vec3::dot(ray.direction, hit_record.normal) / ray.direction.length()
            )
        } else {
            (
                hit_record.normal,
                1.0 / self.ref_idx,
                -Vec3::dot(ray.direction, hit_record.normal) / ray.direction.length()
            )
        };

        let direction = match refract(ray.direction, outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflect_prob = schlick(cosine, self.ref_idx);
                if rand::random::<f32>() < reflect_prob {
                    reflect(ray.direction, hit_record.normal)
                } else {
                    refracted
                }
            }
            None => reflect(ray.direction, hit_record.normal),
        };

        Some(Scattered {
            attenuation: Vec3::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hit_record.point, direction),
        })
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}