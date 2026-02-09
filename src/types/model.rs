use crate::{rasterizer::rasterizer_point::RasterizerPoint, shaders::traits::fragment_shader::FragmentShader, types::{mesh::Mesh, transform::Transform}};

pub struct Model {
    pub name: String,
    pub transform: Transform,
    pub mesh: Mesh,
    pub shader: Box<dyn FragmentShader>,
    pub rasterizer_points: Vec<RasterizerPoint>,
}

impl Model {
    pub fn new(name: &str, mesh: Mesh, shader: Box<dyn FragmentShader>) -> Self {
        Self {
            name: name.to_string(),
            transform: Transform::default(),
            mesh,
            shader,
            rasterizer_points: Vec::new(),
        }
    }
}
