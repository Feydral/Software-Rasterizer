#![allow(dead_code)]

use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, types::texture::Texture};

#[derive(Clone)]
pub struct TextureShader {
    pub texture: Texture,
    pub wireframe: bool,
}

impl TextureShader {
    pub fn new(texture: Texture, wireframe: bool) -> Self {
        Self { 
            texture, 
            wireframe,
        }
    }

    #[inline(always)]
    #[allow(unused_variables)]
    pub fn pixel_color(&self, pixel_coord: Float2, uv: Float2, normal: Float3, depth: f32) -> Float4 {
        let u_frac = uv.x - uv.x.floor();
        let v_frac = uv.y - uv.y.floor();
        let wscale = self.texture.width().saturating_sub(1);
        let hscale = self.texture.height().saturating_sub(1);

        let x = (u_frac * wscale as f32) as u32;
        let y = (v_frac * hscale as f32) as u32;
        self.texture.get_pixel(x, y)
    }
}