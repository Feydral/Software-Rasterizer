#![allow(dead_code)]

use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, types::texture::Texture};

#[derive(Clone)]
pub struct LitTextureShader {
    pub direction_to_light: Float3,
    pub texture: Texture,
    pub wireframe: bool,
}

impl LitTextureShader {
    pub fn new(direction_to_light: Float3, texture: Texture, wireframe: bool) -> Self {
        Self {
            direction_to_light,
            texture,
            wireframe,
        }
    }

    #[inline(always)]
    #[allow(unused_variables)]
    pub fn pixel_color(&self, pixel_coord: Float2, uv: Float2, normal: Float3, depth: f32) -> Float4 {
        let normal = normal.normalize();
        let mut light_intensity =
            (Float3::dot(normal, self.direction_to_light.normalize()) + 1.0) * 0.5;

        light_intensity = 0.4 + (1.0 - 0.4) * light_intensity;

        let u = uv.x.rem_euclid(1.0);
        let v = uv.y.rem_euclid(1.0);

        let width = self.texture.width() as f32;
        let height = self.texture.height() as f32;

        let mut x = (u * width).floor() as i32;
        let mut y = (v * height).floor() as i32;

        x = x.clamp(0, self.texture.width() as i32 - 1);
        y = y.clamp(0, self.texture.height() as i32 - 1);

        let mut color = self.texture.get_pixel(x as u32, y as u32);

        color.x *= light_intensity;
        color.y *= light_intensity;
        color.z *= light_intensity;

        color
    }
}
