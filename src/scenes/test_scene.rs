#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use minifb::Key;

use crate::core::{input, resource_helper};

use crate::math::numerics::float3::Float3;
use crate::math::numerics::float4::Float4;

use crate::rasterizer::camera::Camera;
use crate::rasterizer::rasterizer;
use crate::rasterizer::render_target::RenderTarget;

use crate::shaders::shader_base::Shader;
use crate::shaders::texture_shader::TextureShader;
use crate::shaders::lit_texture_shader::LitTextureShader;
use crate::shaders::transparent_texture_shader::TransparentTextureShader;

use crate::types::mesh::Mesh;
use crate::types::model::Model;
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

            speed: 0.8,
        }
    }

    fn get_model(&mut self, name: &str) -> Option<&mut Model> {
        self.models.iter_mut().find(|model| model.name == name)
    }

    fn create_model(&mut self, name: &str, shader: Shader) -> &mut Model {
        self.models.push(Model::new(name, Mesh::empty(), shader));
        self.models.last_mut().unwrap()
    }
}

impl Scene for TestScene {
    fn start(&mut self, render_target: &mut RenderTarget) {
        #[cfg(feature = "release_paths")]
        let color = resource_helper::load_texture("assets/color.png");
        #[cfg(feature = "release_paths")]
        let dragon_mesh = resource_helper::load_mesh("assets/dragon.obj");

        #[cfg(not(feature = "release_paths"))]
        let color = resource_helper::load_texture("../rasterizer/src/assets/color.png");
        #[cfg(not(feature = "release_paths"))]
        let dragon_mesh = resource_helper::load_mesh("../rasterizer/src/assets/dragon.obj");

        let dragon_model = self.create_model("Dragon", Shader::LitTextureShader(LitTextureShader::new(Float3::UNIT_Y, color)));
        dragon_model.mesh = dragon_mesh;
        dragon_model.transform.set_scale(Float3::new(0.2, 0.2, 0.2));
        dragon_model.transform.set_position(Float3::new(0.0, 0.05, 0.0));

        #[cfg(feature = "release_paths")]
        let floor_texture = resource_helper::load_texture("assets/floortexture.png");
        #[cfg(feature = "release_paths")]
        let floor_mesh = resource_helper::load_mesh("assets/Floor.obj");

        #[cfg(not(feature = "release_paths"))]
        let floor_texture = resource_helper::load_texture("../rasterizer/src/assets/floortexture.png");
        #[cfg(not(feature = "release_paths"))]
        let floor_mesh = resource_helper::load_mesh("../rasterizer/src/assets/Floor.obj");

        let floor_model = self.create_model("Floor", Shader::TextureShader(TextureShader::new(floor_texture)));
        floor_model.mesh = floor_mesh;

        #[cfg(feature = "release_paths")]
        let sw_texture = resource_helper::load_texture("assets/Smallworldtexture.png");
        #[cfg(feature = "release_paths")]
        let sw_mesh = resource_helper::load_mesh("assets/SmallWorld.obj");

        #[cfg(not(feature = "release_paths"))]
        let sw_texture = resource_helper::load_texture("../rasterizer/src/assets/Smallworldtexture.png");
        #[cfg(not(feature = "release_paths"))]
        let sw_mesh = resource_helper::load_mesh("../rasterizer/src/assets/SmallWorld.obj");

        let sw_model = self.create_model("SmallWorld", Shader::LitTextureShader(LitTextureShader::new(Float3::new(0.5, 1.0, 0.3), sw_texture)));
        sw_model.mesh = sw_mesh;
        sw_model.transform.set_position(Float3::new(1.0, 0.01, 0.0));
        sw_model.transform.set_scale(Float3::new(0.2, 0.2, 0.2));

        self.cam.transform.set_position(Float3::new(0.0, 0.2, -1.0));
    }

    fn update(&mut self, delta_time: f32, render_target: &mut RenderTarget) {
        println!("Resolution: {}x{}, Fps: {}", render_target.width(), render_target.height(), 1.0 / delta_time);

        if input::is_pressed(Key::R) {
            self.speed += 0.04;
        }
        else if input::is_pressed(Key::F) {
            self.speed -= 0.04;
        }

        let speed = self.speed * delta_time;
    
        if input::is_pressed(Key::W) {
            self.cam.transform.translate(Float3::new(self.cam.transform.forward().x * speed, 0.0, self.cam.transform.forward().z * speed));
        }
        if input::is_pressed(Key::S) {
            self.cam.transform.translate(Float3::new(self.cam.transform.backward().x * speed, 0.0, self.cam.transform.backward().z * speed));
        }
        if input::is_pressed(Key::D) {
            self.cam.transform.translate(Float3::new(self.cam.transform.right().x * speed, 0.0, self.cam.transform.right().z * speed));
        }
        if input::is_pressed(Key::A) {
            self.cam.transform.translate(Float3::new(self.cam.transform.left().x * speed, 0.0, self.cam.transform.left().z * speed));
        }

        if input::is_pressed(Key::LeftShift) {
            self.cam.transform.translate(Float3::ZERO - Float3::UNIT_Y * speed);
        }
        if input::is_pressed(Key::Space) {
            self.cam.transform.translate(Float3::UNIT_Y * speed);
        }

        let rotation_speed = self.speed * delta_time;

        if input::is_pressed(Key::Left) {
            self.cam.transform.rotate(Float3::new(0.0, rotation_speed, 0.0));
        }
        if input::is_pressed(Key::Right) {
            self.cam.transform.rotate(Float3::new(0.0, -rotation_speed, 0.0));
        }
        if input::is_pressed(Key::Up) {
            self.cam.transform.rotate(Float3::new(rotation_speed, 0.0, 0.0));
        }
        if input::is_pressed(Key::Down) {
            self.cam.transform.rotate(Float3::new(-rotation_speed, 0.0, 0.0));
        }

        render_target.clear(Float4::new(0.53, 0.81, 0.92, 1.0));
        rasterizer::render(render_target, &mut self.models, &self.cam);
    }

    fn resize(&mut self, new_width: u32, new_height: u32, render_target: &mut RenderTarget) {
        
    }
}
