use crate::mesh::Mesh;
use crate::geometry::Vec3;

pub(crate) fn generate_test_mesh(radius: f32, position: Vec3) -> Mesh {
    let mut mesh = Mesh::new();
    let v0 = mesh.push_vertex(Vec3::new(position.x(), position.y() + radius, position.z()));
    let v1 = mesh.push_vertex(Vec3::new(position.x() - radius, position.y(), position.z()));
    let v2 = mesh.push_vertex(Vec3::new(position.x() + radius, position.y(), position.z()));
    let v3 = mesh.push_vertex(Vec3::new(position.x(), position.y(), position.z() + radius));

    mesh.push_face(v0, v1, v2);
    mesh.push_face(v0, v1, v3);
    mesh.push_face(v2, v0, v3);
    mesh.push_face(v1, v2, v3);
    mesh
}