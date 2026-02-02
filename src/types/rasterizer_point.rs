use crate::math::numerics::{float2::Float2, float3::Float3};

pub struct RasterizerPoint {
    pub depth: f32,
    pub screen_pos: Float2,
    pub tex_coords: Float2,
    pub normals: Float3,
}

impl RasterizerPoint {
    pub fn new(depth: f32, screen_pos: Float2) -> Self {
        Self {
            depth,
            screen_pos,
            tex_coords: Float2::ZERO,
            normals: Float3::ZERO,
        }
    }
}