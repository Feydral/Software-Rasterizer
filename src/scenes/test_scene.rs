use minifb::Key;

use crate::engine::input;
use crate::math::numerics::float2::Float2;
use crate::math::numerics::float3::Float3;
use crate::rasterizer::rasterizer;
use crate::types::model::Model;
use crate::types::{scene::Scene, mesh::Mesh};
use crate::rasterizer::render_target::RenderTarget;

pub struct TestScene {
    models: Vec<Model>,

    speed: f32,
}

impl TestScene {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),

            speed: 5.0,
        }
    }

    fn get_model(&mut self, name: &str) -> Option<&mut Model> {
        self.models.iter_mut().find(|model| model.name() == name)
    }

    fn create_model(&mut self, name: &str) -> &mut Model {
        self.models.push(Model::new(name, Mesh::new()));
        self.models.last_mut().unwrap()
    }
}

impl Scene for TestScene {
    fn start(&mut self, render_target: &mut RenderTarget) {
        let model = self.create_model("Model");
        model.mesh_mut().vertices = vec![
            // Front (+Z)
            Float3::new(-0.5, -0.5,  0.5),
            Float3::new( 0.5, -0.5,  0.5),
            Float3::new( 0.5,  0.5,  0.5),

            Float3::new(-0.5, -0.5,  0.5),
            Float3::new( 0.5,  0.5,  0.5),
            Float3::new(-0.5,  0.5,  0.5),

            // Back (-Z)
            Float3::new( 0.5, -0.5, -0.5),
            Float3::new(-0.5, -0.5, -0.5),
            Float3::new(-0.5,  0.5, -0.5),

            Float3::new( 0.5, -0.5, -0.5),
            Float3::new(-0.5,  0.5, -0.5),
            Float3::new( 0.5,  0.5, -0.5),

            // Left (-X)
            Float3::new(-0.5, -0.5, -0.5),
            Float3::new(-0.5, -0.5,  0.5),
            Float3::new(-0.5,  0.5,  0.5),

            Float3::new(-0.5, -0.5, -0.5),
            Float3::new(-0.5,  0.5,  0.5),
            Float3::new(-0.5,  0.5, -0.5),

            // Right (+X)
            Float3::new( 0.5, -0.5,  0.5),
            Float3::new( 0.5, -0.5, -0.5),
            Float3::new( 0.5,  0.5, -0.5),

            Float3::new( 0.5, -0.5,  0.5),
            Float3::new( 0.5,  0.5, -0.5),
            Float3::new( 0.5,  0.5,  0.5),

            // Top (+Y)
            Float3::new(-0.5,  0.5,  0.5),
            Float3::new( 0.5,  0.5,  0.5),
            Float3::new( 0.5,  0.5, -0.5),

            Float3::new(-0.5,  0.5,  0.5),
            Float3::new( 0.5,  0.5, -0.5),
            Float3::new(-0.5,  0.5, -0.5),

            // Bottom (-Y)
            Float3::new(-0.5, -0.5, -0.5),
            Float3::new( 0.5, -0.5, -0.5),
            Float3::new( 0.5, -0.5,  0.5),

            Float3::new(-0.5, -0.5, -0.5),
            Float3::new( 0.5, -0.5,  0.5),
            Float3::new(-0.5, -0.5,  0.5),
        ];
    }

    fn update(&mut self, delta_time: f32, render_target: &mut RenderTarget) {
        println!("Fps: {}", 1.0 / delta_time);

        if input::is_pressed(Key::R) {
            self.speed += 0.1;
        }
        else if input::is_pressed(Key::F) {
            self.speed -= 0.1;
        }

        let speed = self.speed * delta_time;
    
        let model = self.get_model("Model").unwrap();

        model.tranform_mut().rotate(Float3::new( speed * 0.1, speed * 0.5, 0.0));

        if input::is_pressed(Key::W) {
            model.tranform_mut().translate(Float3::new(0.0, 0.0, speed));
        }
        if input::is_pressed(Key::S) {
            model.tranform_mut().translate(Float3::new(0.0, 0.0, -speed));
        }
        if input::is_pressed(Key::D) {
            model.tranform_mut().translate(Float3::new(speed, 0.0, 0.0));
        }
        if input::is_pressed(Key::A) {
            model.tranform_mut().translate(Float3::new(-speed, 0.0, 0.0));
        }

        render_target.clear();
        rasterizer::render(render_target, &self.models);
    }

    fn resize(&mut self, new_width: u32, new_height: u32, render_target: &mut RenderTarget) {
        *render_target = RenderTarget::new(new_width, new_height);
    }
}
