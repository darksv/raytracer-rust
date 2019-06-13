use crate::mesh::{Mesh, VertexIndex, MeshBuilder, NormalIndex};
use crate::geometry::Vec3;
use std::path::Path;
use wavefront_obj::obj::{Shape, Primitive};

pub(crate) fn generate_test_mesh(radius: f32, position: Vec3) -> Mesh {
    let mut builder = MeshBuilder::new();
    let v0 = builder.push_vertex(Vec3::new(position.x(), position.y() + radius, position.z()));
    let v1 = builder.push_vertex(Vec3::new(position.x() - radius, position.y(), position.z()));
    let v2 = builder.push_vertex(Vec3::new(position.x() + radius, position.y(), position.z()));
    let v3 = builder.push_vertex(Vec3::new(position.x(), position.y(), position.z() + radius));
    let n = builder.push_normal(Vec3::zeros());
    builder.push_face(v0, n, v1, n, v2, n);
    builder.push_face(v0, n, v1, n, v3, n);
    builder.push_face(v2, n, v0, n, v3, n);
    builder.push_face(v1, n, v2, n, v3, n);
    builder.build()
}

pub(crate) fn load_obj<P: AsRef<Path>>(path: P) -> Mesh {
    let content = std::fs::read_to_string(path).unwrap();
    let obj = wavefront_obj::obj::parse(content).unwrap();
    let object = &obj.objects[0];
    let mut mesh = MeshBuilder::new();

    let vertices: Vec<VertexIndex> = object.vertices.iter().enumerate()
        .map(|(i, v)| mesh.push_vertex(Vec3::new(v.x as f32, v.y as f32, v.z as f32)))
        .collect();

    let normals: Vec<NormalIndex> = object.normals.iter()
        .map(|n| mesh.push_normal(Vec3::new(n.x as f32, n.y as f32, n.z as f32)))
        .collect();
    let n0 = mesh.push_normal(Vec3::zeros());

    let get_normal = |n: Option<usize>| n.and_then(|it| normals.get(it).copied()).unwrap_or(n0);

    for g in &object.geometry {
        for shape in &g.shapes {
            match shape.primitive {
                Primitive::Triangle(
                    (v0, _, n0),
                    (v1, _, n1),
                    (v2, _, n2)
                ) => {
                    mesh.push_face(
                        vertices[v0], get_normal(n0),
                        vertices[v1], get_normal(n1),
                        vertices[v2], get_normal(n2),
                    );
                }
                _ => unimplemented!()
            }
        }
    }
    mesh.build()
}