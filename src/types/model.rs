use crate::types::{mesh::Mesh, transform::Transform};

pub struct Model {
    name: String,
    transform: Transform,
    mesh: Mesh,
}

impl Model {
    pub fn new(name: &str, mesh: Mesh) -> Self {
        Self {
            name: name.to_string(),
            transform: Transform::default(),
            mesh,
        }
    }

    pub fn tranform(&self) -> &Transform {
        &self.transform
    }

    pub fn tranform_mut(&mut self) -> &mut Transform {
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
}