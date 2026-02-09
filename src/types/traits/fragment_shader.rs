use crate::math::numerics::{float2::Float2, float3::Float3, float4::Float4};

pub trait FragmentShader: Send + Sync {
    fn pixel_color(&self, pixel_coord: Float2, uv: Float2, normal: Float3, depth: f32) -> Float4;
}