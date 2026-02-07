use crate::types::{mesh::Mesh, rasterizer_point::RasterizerPoint, transform::Transform};

pub struct Model {
    pub name: String,
    pub transform: Transform,
    pub mesh: Mesh,
}

impl Model {
    pub fn new(name: &str, mesh: Mesh) -> Self {
        Self {
            name: name.to_string(),
            transform: Transform::default(),
            mesh,
        }
    }
}