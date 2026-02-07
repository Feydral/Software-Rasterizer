use crate::math::numerics::float3::Float3;

pub struct Tile {
    pub x: u32,
    pub y: u32,

    pub width: u32,
    pub height: u32,

    pub color_buffer: Vec<Float3>,
    pub depth_buffer: Vec<f32>,

    pub triangles: Vec<u32>,
}