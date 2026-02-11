use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, shaders::{lit_texture_shader::LitTextureShader, texture_shader::TextureShader, transparent_texture_shader::TransparentTextureShader}};

#[allow(dead_code)]
pub enum Shader {
    TextureShader(TextureShader),
    LitTextureShader(LitTextureShader),
    TransparentTextureShader(TransparentTextureShader),
}

impl Shader {
    #[inline(always)]
    pub fn pixel_color(&self, pixel_coord: Float2, uv: Float2, normal: Float3, depth: f32) -> Float4 {
        match self {
            Shader::TextureShader(s) => s.pixel_color(pixel_coord, uv, normal, depth),
            Shader::LitTextureShader(s) => s.pixel_color(pixel_coord, uv, normal, depth),
            Shader::TransparentTextureShader(s) => s.pixel_color(pixel_coord, uv, normal, depth),
        }
    }

    #[inline(always)]
    pub fn wire_frame(&self) -> bool {
        match self {
            Shader::TextureShader(s) => s.wire_frame(),
            Shader::LitTextureShader(s) => s.wire_frame(),
            Shader::TransparentTextureShader(_) => false,
        }
    }
}