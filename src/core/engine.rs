use std::time::Instant;
use minifb::{Window, WindowOptions};

use crate::{core::input, math::mathf, rasterizer::render_target::RenderTarget, types::traits::scene::Scene};

pub struct Engine {
    width: u32,
    height: u32,
    window: Window,
    framebuffer: Vec<u32>,
    last_frame: Instant,
}

impl Engine {
    pub const START_WIDTH: u32 = 1408;
    pub const START_HEIGHT: u32 = 792;
    pub const TITLE: &'static str = "Software Rasterizer";

    pub fn new() -> Self {
        let window = Window::new(
            Self::TITLE,
            Self::START_WIDTH as usize,
            Self::START_HEIGHT as usize,
            WindowOptions {
                resize: true,
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::Stretch,
                ..WindowOptions::default()
            },
        )
        .expect("Failed to create window");

        let framebuffer = vec![0u32; Self::START_WIDTH as usize * Self::START_HEIGHT as usize];

        Self {
            width: Self::START_WIDTH,
            height: Self::START_HEIGHT,
            window,
            framebuffer,
            last_frame: Instant::now(),
        }
    }

    pub fn run<T: Scene>(&mut self, scene: &mut T) {
        let mut render_target = RenderTarget::new(Self::START_WIDTH, Self::START_HEIGHT);

        scene.start(&mut render_target);
        while self.window.is_open() {
            let new_width = self.window.get_size().0 as u32;
            let new_height = self.window.get_size().1 as u32;

            if new_width != self.width || new_height != self.height {
                self.width = new_width;
                self.height = new_height;
                self.resize(scene, &mut render_target);
            }

            input::update(&self.window);

            let now = Instant::now();
            let delta_time = (now - self.last_frame).as_secs_f32();
            self.last_frame = now;

            scene.update(delta_time, &mut render_target);
            self.render(&mut render_target);

            self.window
                .update_with_buffer(
                    &self.framebuffer,
                    self.width as usize,
                    self.height as usize,
                )
                .unwrap();
        }
    }

    fn render(&mut self, render_target: &RenderTarget) {
        for y in 0..self.height {
            for x in 0..self.width {
                let r = mathf::clamp(render_target.get_pixel_color(x, y).x, 0.0, 1.0) * 255.0;
                let g = mathf::clamp(render_target.get_pixel_color(x, y).y, 0.0, 1.0) * 255.0;
                let b = mathf::clamp(render_target.get_pixel_color(x, y).z, 0.0, 1.0) * 255.0;

                let color = ((r as u32) << 16) | ((g as u32) << 8) | ((b as u32) << 0);

                let flipped_y = self.height - 1 - y;
                self.framebuffer[(flipped_y * self.width + x) as usize] = color; 
            }
        }
    }

    fn resize<T: Scene>(&mut self, scene: &mut T, render_target: &mut RenderTarget) {
        self.framebuffer = vec![0u32; (self.width * self.height) as usize];
        *render_target = RenderTarget::new(self.width, self.height);
        scene.resize(self.width, self.height, render_target);
    }
}