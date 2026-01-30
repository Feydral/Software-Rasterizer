use crate::math::numerics::float3::Float3;
use crate::math::mathi;

pub struct RenderTarget {
    color_buffer: Vec<Float3>,
    depth_buffer: Vec<f32>,

    width: u32,
    height: u32,
}

impl RenderTarget {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            color_buffer: vec![Float3::new(0.0, 0.0, 0.0); (width * height) as usize],
            depth_buffer: vec![0.0; (width * height) as usize],

            width,
            height,
        }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.color_buffer.fill(Float3::new(0.0, 0.0, 0.0));
        self.depth_buffer.fill(0.0);
        self
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Float3, depth: f32) {
        let index = mathi::xy_to_index(x, y, self.width, self.height) as usize;
        self.color_buffer[index] = color;
        self.depth_buffer[index] = depth;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> (Float3, f32) {
        let index = mathi::xy_to_index(x, y, self.width, self.height) as usize;
        (self.color_buffer[index], self.depth_buffer[index])
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}