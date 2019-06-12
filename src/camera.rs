use crate::geometry::Vec3;
use crate::ray::Ray;

pub(crate) struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub(crate) fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
        }
    }
}


impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
        }
    }
}