use crate::geometry::Vec3;

pub(crate) struct Ray {
    pub(crate) origin: Vec3,
    pub(crate) direction: Vec3,
}

impl Ray {
    pub(crate) fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub(crate) fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}