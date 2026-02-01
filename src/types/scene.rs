use crate::rasterizer::render_target::RenderTarget;

pub trait Scene {
    fn resize(&mut self, new_width: u32, new_height: u32, render_target: &mut RenderTarget);
    fn update(&mut self, delta_time: f32, render_target: &mut RenderTarget);
    fn start(&mut self, render_target: &mut RenderTarget);
}