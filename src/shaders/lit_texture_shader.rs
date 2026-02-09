#![allow(dead_code)]

use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, types::{texture::Texture, traits::fragment_shader::FragmentShader}};

#[derive(Clone)]
pub struct LitTextureShader {
    pub direction_to_light: Float3,
    pub texture: Texture,
}

impl LitTextureShader {
    pub fn new(direction_to_light: Float3, texture: Texture) -> Self {
        Self {
            direction_to_light,
            texture,
        }
    }
}

impl FragmentShader for LitTextureShader {
    #[inline(always)]
    #[allow(unused_variables)]
    fn pixel_color(&self, pixel_coord: Float2, uv: Float2, normal: Float3, depth: f32) -> Float4 {
        let normal = normal.normalize();

        let mut light_intensity = (Float3::dot(normal, self.direction_to_light) + 1.0) * 0.5;
        light_intensity = 0.4 + (1.0 - 0.4) * light_intensity;

        let u_frac = uv.x - uv.x.floor();
        let v_frac = uv.y - uv.y.floor();

        let wscale = self.texture.width().saturating_sub(1);
        let hscale = self.texture.height().saturating_sub(1);

        let x = (u_frac * wscale as f32) as u32;
        let y = (v_frac * hscale as f32) as u32;

        let mut color = self.texture.get_pixel(x, y);

        // Lichtintensität anwenden (RGB-Kanäle)
        color.x *= light_intensity;
        color.y *= light_intensity;
        color.z *= light_intensity;

        color
    }
}
