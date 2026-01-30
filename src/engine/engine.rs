use std::time::Instant;
use minifb::{Key, Window, WindowOptions};

use crate::{rasterizer::render_target::RenderTarget, types::scene::Scene};

pub struct Engine {
    window: Window,
    framebuffer: Vec<u32>,
    last_frame: Instant,
}

impl Engine {
    pub const WIDTH: u32 = 768;
    pub const HEIGHT: u32 = 768;
    pub const TITLE: &'static str = "Software Rasterizer (minifb)";

    pub fn new() -> Self {
        let window = Window::new(
            Self::TITLE,
            Self::WIDTH as usize,
            Self::HEIGHT as usize,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X1,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .expect("Failed to create window");

        let framebuffer = vec![0u32; Self::WIDTH as usize * Self::HEIGHT as usize];

        Self {
            window,
            framebuffer,
            last_frame: Instant::now(),
        }
    }

    pub fn run<T: Scene>(&mut self, scene: &mut T) {
        let mut render_target = RenderTarget::new(Self::WIDTH, Self::HEIGHT);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = Instant::now();
            let delta_time = (now - self.last_frame).as_secs_f32();
            self.last_frame = now;

            scene.update(delta_time, &mut render_target);
            self.render(&mut render_target);

            self.window
                .update_with_buffer(
                    &self.framebuffer,
                    Self::WIDTH as usize,
                    Self::HEIGHT as usize,
                )
                .unwrap();
        }
    }

    fn render(&mut self, render_target: &RenderTarget) {
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let pixel = render_target.get_pixel(x, y).0 * 255 as f32;

                let color = ((pixel.x as u32) << 16) | ((pixel.y as u32) << 8) | ((pixel.z as u32) << 0);

                let flipped_y = Self::HEIGHT - 1 - y;
                self.framebuffer[(flipped_y * Self::WIDTH + x) as usize] = color;
            }
        }
    }
}