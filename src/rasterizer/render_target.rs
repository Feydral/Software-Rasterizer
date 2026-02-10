use std::f32;

use crate::math::mathi;
use crate::math::numerics::float4::Float4;

pub struct RenderTarget {
    color_buffer: Vec<Float4>,
    depth_buffer: Vec<f32>,

    width: u32,
    height: u32,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            color_buffer: vec![Float4::ZERO; (width * height) as usize],
            depth_buffer: vec![f32::INFINITY; (width * height) as usize],

            width,
            height,
        }
    }

    pub fn clear(&mut self, color: Float4) -> &mut Self {
        self.color_buffer.fill(color);
        self.depth_buffer.fill(f32::INFINITY);
        self
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Float4, depth: f32) {
        if color.w < f32::EPSILON {
            return;
        }

        let index = mathi::xy_to_index(x, y, self.width, self.height) as usize;
        
        let dst = self.color_buffer[index];
        let src = color;
    
        let alpha = src.w.clamp(0.0, 1.0);
        let inv_alpha = 1.0 - alpha;
    
        let out = Float4 {
            x: src.x * alpha + dst.x * inv_alpha,
            y: src.y * alpha + dst.y * inv_alpha,
            z: src.z * alpha + dst.z * inv_alpha,
            w: alpha + dst.w * inv_alpha,
        };
    
        self.color_buffer[index] = out;
        self.depth_buffer[index] = depth;
    }


    pub fn get_pixel_color(&self, x: u32, y: u32) -> Float4 {
        let index = mathi::xy_to_index(x, y, self.width, self.height) as usize;
        self.color_buffer[index]
    }
    
    pub fn get_pixel_depth(&self, x: u32, y: u32) -> f32 {
        let index = mathi::xy_to_index(x, y, self.width, self.height) as usize;
        self.depth_buffer[index]
    }

    pub fn color_buffer(&self) -> &[Float4] {
        self.color_buffer.as_slice()
    }


    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}