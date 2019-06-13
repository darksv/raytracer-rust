use crate::geometry::Vec3;
use crate::ray::Ray;

pub(crate) struct Mesh {
    vertices: Vec<Vec3>,
    triangles: Vec<(u32, u32, u32)>,
    aabb: Aabb,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub(crate) struct Aabb {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32,
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            min_x: std::f32::MAX,
            max_x: std::f32::MIN,
            min_y: std::f32::MAX,
            max_y: std::f32::MIN,
            min_z: std::f32::MAX,
            max_z: std::f32::MIN,
        }
    }
}

pub(crate) struct MeshBuilder {
    vertices: Vec<Vec3>,
    triangles: Vec<(u32, u32, u32)>,
}

impl MeshBuilder {
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            triangles: vec![],
        }
    }

    pub(crate) fn push_vertex(&mut self, v: Vec3) -> VertexIndex {
        let idx = self.vertices.len() as u32;
        self.vertices.push(v);
        VertexIndex(idx)
    }

    pub(crate) fn push_face(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) {
        self.triangles.push((v0.0, v1.0, v2.0));
    }

    pub(crate) fn build(self) -> Mesh {
        let mut aabb = Aabb::default();
        for v in &self.vertices {
            aabb.min_x = aabb.min_x.min(v.x());
            aabb.max_x = aabb.max_x.max(v.x());
            aabb.min_y = aabb.min_y.min(v.y());
            aabb.max_y = aabb.max_y.max(v.y());
            aabb.min_z = aabb.min_z.min(v.z());
            aabb.max_z = aabb.max_z.max(v.z());
        }

        dbg!(&aabb);

        Mesh {
            vertices: self.vertices,
            triangles: self.triangles,
            aabb,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct VertexIndex(u32);

impl Mesh {
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            triangles: vec![],
            aabb: Aabb::default(),
        }
    }

    pub(crate) fn aabb(&self) -> Aabb {
        self.aabb
    }

    pub(crate) fn vertices(&self) -> &[Vec3] {
        &self.vertices
    }

    pub(crate) fn triangles(&self) -> &[(u32, u32, u32)] {
        &self.triangles
    }

    pub(crate) fn iter_triangles<'a>(&'a self) -> impl Iterator<Item=(Vec3, Vec3, Vec3)> + 'a {
        self.triangles
            .iter()
            .map(move |&(i0, i1, i2)| (
                self.vertices[i0 as usize],
                self.vertices[i1 as usize],
                self.vertices[i2 as usize]
            ))
    }
}

pub(crate) fn box_intersect(ray: &Ray, aabb: &Aabb) -> bool {
    // Based on
    // https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-box-intersection

    let mut tmin = (aabb.min_x - ray.origin.x()) / ray.direction.x();
    let mut tmax = (aabb.max_x - ray.origin.x()) / ray.direction.x();

    if tmin > tmax {
        std::mem::swap(&mut tmin, &mut tmax);
    }

    let mut tymin = (aabb.min_y - ray.origin.y()) / ray.direction.y();
    let mut tymax = (aabb.max_y - ray.origin.y()) / ray.direction.y();

    if tymin > tymax {
        std::mem::swap(&mut tymin, &mut tymax);
    }

    if tmin > tymax || tymin > tmax {
        return false;
    }

    if tymin > tmin {
        tmin = tymin;
    }

    if tymax < tmax {
        tmax = tymax;
    }

    let mut tzmin = (aabb.min_z - ray.origin.z()) / ray.direction.z();
    let mut tzmax = (aabb.max_z - ray.origin.z()) / ray.direction.z();

    if tzmin > tzmax {
        std::mem::swap(&mut tzmin, &mut tzmax);
    }

    if tmin > tzmax || tzmin > tmax {
        return false;
    }
//
//    if tzmin > tmin {
//        tmin = tzmin;
//    }
//
//    if tzmax < tmax {
//        tmax = tzmax;
//    }

    true
}