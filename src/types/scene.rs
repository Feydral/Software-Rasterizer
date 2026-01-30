use crate::rasterizer::render_target::RenderTarget;

pub trait Scene {
    fn update(&mut self, delta_time: f32, render_target: &mut RenderTarget);
}