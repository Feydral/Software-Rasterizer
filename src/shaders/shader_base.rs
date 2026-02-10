use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, shaders::{lit_texture_shader::LitTextureShader, texture_shader::TextureShader}};

pub enum Shader {
    TextureShader(TextureShader),
    LitTextureShader(LitTextureShader),
}

impl Shader {
    #[inline(always)]
    pub fn pixel_color(&self, pixel_coord: Float2, uv: Float2, normal: Float3, depth: f32) -> Float4 {
        match self {
            Shader::TextureShader(s) => s.pixel_color(pixel_coord, uv, normal, depth),
            Shader::LitTextureShader(s) => s.pixel_color(pixel_coord, uv, normal, depth),
        }
    }
}