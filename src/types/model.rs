use crate::types::{mesh::Mesh, rasterizer_point::RasterizerPoint, transform::Transform};

pub struct Model {
    name: String,
    transform: Transform,
    mesh: Mesh,
    rasterizer_points: Vec<RasterizerPoint>,
}

impl Model {
    pub fn new(name: &str, mesh: Mesh) -> Self {
        Self {
            name: name.to_string(),
            transform: Transform::default(),
            mesh,
            rasterizer_points: Vec::new(),
        }
    }

    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn mesh_mut(&mut self) -> &mut Mesh {
        &mut self.mesh
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn rasterizer_points(&self) -> &Vec<RasterizerPoint> {
        &self.rasterizer_points
    }

    pub fn rasterizer_points_mut(&mut self) -> &mut Vec<RasterizerPoint> {
        &mut self.rasterizer_points
    }
}