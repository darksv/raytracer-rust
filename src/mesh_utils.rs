use crate::mesh::{Mesh, VertexIndex, MeshBuilder};
use crate::geometry::Vec3;
use std::path::Path;
use wavefront_obj::obj::{Shape, Primitive};

pub(crate) fn generate_test_mesh(radius: f32, position: Vec3) -> Mesh {
    let mut builder = MeshBuilder::new();
    let v0 = builder.push_vertex(Vec3::new(position.x(), position.y() + radius, position.z()));
    let v1 = builder.push_vertex(Vec3::new(position.x() - radius, position.y(), position.z()));
    let v2 = builder.push_vertex(Vec3::new(position.x() + radius, position.y(), position.z()));
    let v3 = builder.push_vertex(Vec3::new(position.x(), position.y(), position.z() + radius));

    builder.push_face(v0, v1, v2);
    builder.push_face(v0, v1, v3);
    builder.push_face(v2, v0, v3);
    builder.push_face(v1, v2, v3);
    builder.build()
}

pub(crate) fn load_obj<P: AsRef<Path>>(path: P) -> Mesh {
    let content = std::fs::read_to_string(path).unwrap();
    let obj = wavefront_obj::obj::parse(content).unwrap();
    let object = &obj.objects[0];
    let mut mesh = MeshBuilder::new();

    let vertices: Vec<VertexIndex> = object.vertices.iter().map(|it| mesh.push_vertex(Vec3::new(it.x as f32, it.y as f32, it.z as f32))).collect();

    for g in &object.geometry {
        for shape in &g.shapes {
            match shape.primitive {
                Primitive::Triangle(v0, v1, v2) => {
                    let v0 = vertices[v0.0];
                    let v1 = vertices[v1.0];
                    let v2 = vertices[v2.0];
                    mesh.push_face(v0, v1, v2);
                }
                _ => unimplemented!()
            }
        }
    }
    mesh.build()
}