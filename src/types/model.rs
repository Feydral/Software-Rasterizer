use crate::{rasterizer::rasterizer_point::RasterizerPoint, shaders::shader_base::Shader, types::{mesh::Mesh, transform::Transform}};

pub struct Model {
    pub name: String,
    pub transform: Transform,
    pub mesh: Mesh,
    pub shader: Shader,
    pub rasterizer_points: Vec<RasterizerPoint>,
}

impl Model {
    pub fn new(name: &str, mesh: Mesh, shader: Shader) -> Self {
        Self {
            name: name.to_string(),
            transform: Transform::default(),
            mesh,
            shader,
            rasterizer_points: Vec::new(),
        }
    }
}
