use std::io::Write;
use std::error::Error;
use rayon::prelude::*;

mod geometry;
mod ray;
mod hitable;
mod hitable_list;
mod sphere;
mod camera;
mod material;
mod triangulated_model;
mod mesh;
mod mesh_utils;

use crate::geometry::Vec3;
use crate::ray::Ray;
use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::material::{ScatterInfo, Lambertian, Metal, Dielectric};
use std::sync::Arc;
use crate::triangulated_model::TriangulatedModel;
use crate::mesh_utils::load_obj;

fn color(ray: &Ray, hitable: &dyn Hitable, depth: usize) -> Vec3 {
    match hitable.hit(ray, 0.001, std::f32::INFINITY) {
        Some(record) => {
            if depth < 50 {
                if let Some(ScatterInfo { attenuation, ref scattered }) = record.material.scatter(ray, &record) {
                    return attenuation * color(scattered, hitable, depth + 1);
                }
            }
            Vec3::new(0.0, 0.0, 0.0)
        }
        None => {
            let uv = ray.direction.normalize();
            let t = 0.5 * (uv.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(rand::random(), rand::random(), rand::random()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_len() >= 1.0 {
            return p;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let m = load_obj(r"sample.obj");

    let width = 1920;
    let height = 1080;
    let ns = 100;

    let hitables = HitableList::from_vec(vec![
        Box::new(TriangulatedModel::new(
            m,
            Arc::new(
                Lambertian { albedo: Vec3::new(0.8, 0.3, 0.3) }
            ),
        )),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            material: Arc::new(
                Lambertian { albedo: Vec3::new(0.8, 0.3, 0.0) }
            ),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(
                Metal { albedo: Vec3::new(0.8, 0.6, 0.2) }
            ),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
            material: Arc::new(
                Dielectric { ref_idx: 1.5 }
            ),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: -0.45,
            material: Arc::new(
                Dielectric { ref_idx: 1.5 }
            ),
        }),
    ]);

    let camera = Camera::default();

    let mut frame_buffer = vec![Vec3::new(0.0, 0.0, 0.0); width * height];
    frame_buffer.par_iter_mut().enumerate().for_each(|(n, pixel)| {
        let i = n % width;
        let j = height - n / width;

        let mut col = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..ns {
            let u = (i as f32 + rand::random::<f32>()) / width as f32;
            let v = (j as f32 + rand::random::<f32>()) / height as f32;
            let r = camera.ray(u, v);
            col = col + color(&r, &hitables, 0);
        }
        let c = col / ns as f32;
        *pixel = Vec3::new(
            c.x().sqrt(),
            c.y().sqrt(),
            c.z().sqrt(),
        );
    });

    let mut writer = std::io::BufWriter::new(std::fs::File::create("image.ppm")?);
    write!(writer, "P3\n")?;
    write!(writer, "{} {}\n", width, height)?;
    write!(writer, "{}\n", 255)?;
    for Vec3 { raw: [r, g, b] } in frame_buffer {
        let r = (255.99 * r) as i32;
        let g = (255.99 * g) as i32;
        let b = (255.99 * b) as i32;

        write!(writer, "{} {} {}\n", r, g, b)?;
    }

    Ok(())
}
