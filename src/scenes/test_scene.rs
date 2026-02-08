#![allow(unused_variables)]

use minifb::Key;

use crate::core::input;
use crate::math::numerics::float3::Float3;
use crate::rasterizer::camera::Camera;
use crate::rasterizer::rasterizer;
use crate::types::mesh::Mesh;
use crate::types::model::Model;
use crate::rasterizer::render_target::RenderTarget;
use crate::types::traits::scene::Scene;

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

    fn create_model(&mut self, name: &str) -> &mut Model {
        self.models.push(Model::new(name, Mesh::new()));
        self.models.last_mut().unwrap()
    }
}

impl Scene for TestScene {
    fn start(&mut self, render_target: &mut RenderTarget) {
        let model = self.create_model("Model");// 8 Würfelecken
        model.mesh.vertices = vec![
            Float3::new(-0.5, -0.5,  0.5), // 0: vorne unten links
            Float3::new( 0.5, -0.5,  0.5), // 1: vorne unten rechts
            Float3::new( 0.5,  0.5,  0.5), // 2: vorne oben rechts
            Float3::new(-0.5,  0.5,  0.5), // 3: vorne oben links
            Float3::new(-0.5, -0.5, -0.5), // 4: hinten unten links
            Float3::new( 0.5, -0.5, -0.5), // 5: hinten unten rechts
            Float3::new( 0.5,  0.5, -0.5), // 6: hinten oben rechts
            Float3::new(-0.5,  0.5, -0.5), // 7: hinten oben links
        ];

        // Indices für 12 Dreiecke (2 pro Seite)
        model.mesh.indices = vec![
            // Front (+Z)
            0, 1, 2,  0, 2, 3,
            // Back (-Z)
            5, 4, 7,  5, 7, 6,
            // Left (-X)
            4, 0, 3,  4, 3, 7,
            // Right (+X)
            1, 5, 6,  1, 6, 2,
            // Top (+Y)
            3, 2, 6,  3, 6, 7,
            // Bottom (-Y)
            4, 5, 1,  4, 1, 0,
        ];

        let floor = self.create_model("Floor");
        
        let size: f32 = 100.0;
        let divisions: i32 = 20;
        let half = size / 2.0;
        let step = size / divisions as f32;

        let verts_per_row = divisions + 1;

        let mut vertices = Vec::with_capacity((verts_per_row * verts_per_row) as usize);
        let mut indices  = Vec::with_capacity((divisions * divisions * 6) as usize);

        // Vertices
        for z in 0..=divisions {
            for x in 0..=divisions {
                let px = -half + x as f32 * step;
                let pz = -half + z as f32 * step;
                vertices.push(Float3::new(px, -2.0, pz));
            }
        }

        // Indices
        for z in 0..divisions {
            for x in 0..divisions {
                let i0 =  z * verts_per_row + x;
                let i1 =  i0 + 1;
                let i2 = (z + 1) * verts_per_row + x;
                let i3 =  i2 + 1;
            
                // Triangle 1
                indices.push(i0 as u32);
                indices.push(i3 as u32);
                indices.push(i1 as u32);
            
                // Triangle 2
                indices.push(i0 as u32);
                indices.push(i2 as u32);
                indices.push(i3 as u32);
            }
        }

        floor.mesh.vertices = vertices;
        floor.mesh.indices  = indices;
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
    
        self.get_model("Model").unwrap().transform.rotate(Float3::new( speed * 0.1, speed * 0.5, 0.0));

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

        render_target.clear();
        rasterizer::render(render_target, &mut self.models, &self.cam);
    }

    fn resize(&mut self, new_width: u32, new_height: u32, render_target: &mut RenderTarget) {
        
    }
}
