use crate::geometry::Vec3;

pub(crate) struct Mesh {
    vertices: Vec<Vec3>,
    triangles: Vec<(u32, u32, u32)>,
}

#[derive(Copy, Clone)]
pub(crate) struct VertexIndex(u32);

impl Mesh {
    pub(crate) fn new() -> Self {
        Self {
            vertices: vec![],
            triangles: vec![],
        }
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

    pub(crate) fn push_vertex(&mut self, v: Vec3) -> VertexIndex {
        let idx = self.vertices.len() as u32;
        self.vertices.push(v);
        VertexIndex(idx)
    }

    pub(crate) fn push_face(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) {
        self.triangles.push((v0.0, v1.0, v2.0));
    }
}

