use crate::math::numerics::float2::Float2;
use crate::math::numerics::float3::Float3;
use crate::rasterizer::rasterizer;
use crate::types::{scene::Scene, mesh::Mesh};
use crate::rasterizer::render_target::RenderTarget;

pub struct TestScene {
    vertices: Vec<Float3>,
}

impl TestScene {
    pub fn new() -> Self {
        Self {
            vertices: vec![Float3::new(0.0, 0.0, 0.0), Float3::new(768.0, 768.0, 0.0), Float3::new(768.0, 0.0, 0.0)],
        }
    }
}

impl Scene for TestScene {
    fn update(&mut self, delta_time: f32, render_target: &mut RenderTarget) {
        println!("{}", 1.0 / delta_time);

        let mesh = vec![Mesh {vertices: self.vertices.clone(), indices: vec![0], normals: vec![Float3::new(0.0, 0.0, 0.0)], uvs: vec![Float2::new(0.0, 0.0)]}];

        render_target.clear();
        rasterizer::render(render_target, &mesh);
    }
}
