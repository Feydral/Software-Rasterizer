use crate::math::{mathi, numerics::float4::Float4};

#[derive(Clone)]
pub struct Texture {
    image: Vec<Float4>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(image: Vec<Vec<Float4>>) -> Self {
        Texture {
            image: image.clone().into_iter().flatten().collect(),
            width: image.len() as u32,
            height: image[0].len() as u32,
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Float4 {
        self.image[mathi::xy_to_index(x, y, self.width, self.height) as usize]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}