#![allow(unused_variables)]

use minifb::Key;

use crate::core::{input, resource_helper};
use crate::math::numerics::float3::Float3;
use crate::math::numerics::float4::Float4;
use crate::rasterizer::camera::Camera;
use crate::rasterizer::rasterizer;
use crate::shaders::lit_texture_shader::LitTextureShader;
use crate::shaders::traits::fragment_shader::FragmentShader;
use crate::types::mesh::Mesh;
use crate::types::model::Model;
use crate::rasterizer::render_target::RenderTarget;
use crate::types::scene::Scene;

pub struct TestScene {
    models: Vec<Model>,
    cam: Camera,

    speed: f32,
}

impl TestScene {
    pub fn new() -> Self {
        Self {
            models: Vec::new(),
            cam: Camera::new(100.0),

            speed: 5.0,
        }
    }

    fn get_model(&mut self, name: &str) -> Option<&mut Model> {
        self.models.iter_mut().find(|model| model.name == name)
    }

    fn create_model(&mut self, name: &str, shader: Box<dyn FragmentShader>) {
        self.models.push(Model::new(name, Mesh::empty(), shader));
    }
}

impl Scene for TestScene {
    fn start(&mut self, render_target: &mut RenderTarget) {
        let color = resource_helper::load_texture("C:/Users/lianh/Development/rasterizer/src/assets/color.png");
        let mesh = resource_helper::load_mesh("C:/Users/lianh/Development/rasterizer/src/assets/dragon.obj");

        self.create_model("Dragon", Box::new(LitTextureShader::new(Float3::UNIT_Y, color)));
        let model = self.get_model("Dragon").unwrap();
        model.mesh = mesh;
        model.transform.set_position(Float3::new(0.0, -1.5, 0.0));
    }

    fn update(&mut self, delta_time: f32, render_target: &mut RenderTarget) {
        println!("Fps: {}", 1.0 / delta_time);

        if input::is_pressed(Key::R) {
            self.speed += 0.3;
        }
        else if input::is_pressed(Key::F) {
            self.speed -= 0.3;
        }

        let speed = self.speed * delta_time;
    
        self.get_model("Dragon").unwrap().transform.rotate(Float3::new( 0.0, speed * 0.3, 0.0));

        if input::is_pressed(Key::W) {
            self.cam.transform.translate(Float3::new(0.0, 0.0, speed));
        }
        if input::is_pressed(Key::S) {
            self.cam.transform.translate(Float3::new(0.0, 0.0, -speed));
        }
        if input::is_pressed(Key::D) {
            self.cam.transform.translate(Float3::new(speed, 0.0, 0.0));
        }
        if input::is_pressed(Key::A) {
            self.cam.transform.translate(Float3::new(-speed, 0.0, 0.0));
        }

        render_target.clear(Float4::new(0.53, 0.81, 0.92, 1.0));
        rasterizer::render(render_target, &mut self.models, &self.cam);
    }

    fn resize(&mut self, new_width: u32, new_height: u32, render_target: &mut RenderTarget) {
        
    }
}
