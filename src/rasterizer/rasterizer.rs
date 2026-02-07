use crate::{math::numerics::{float2::Float2, float3::Float3}, rasterizer::{camera::Camera, render_target::RenderTarget}, types::{model::Model, rasterizer_model::RasterizerModel, rasterizer_point::RasterizerPoint, transform::Transform}};
use crate::math::mathf as f;
use crate::math::mathi as i;

pub fn render(render_target: &mut RenderTarget, data: &mut Vec<Model>, cam: &Camera) {
    let mut models: Vec<RasterizerModel> = Vec::new();
    for model_raw in data {
        models.push(RasterizerModel::process_model(model_raw, render_target, cam));
    }

    for model in models {
        for i in (0..model.points.len()).step_by(3) {
            let r0 = &model.points[i + 0];
            let r1 = &model.points[i + 1];
            let r2 = &model.points[i + 2];

            let a = r0.screen_pos;
            let b = r1.screen_pos;
            let c = r2.screen_pos;

            // Triangle bounds
            let min_x = f::min(a.x, f::min(b.x, c.x));
            let min_y = f::min(a.y, f::min(b.y, c.y));
            let max_x = f::max(a.x, f::max(b.x, c.x));
            let max_y = f::max(a.y, f::max(b.y, c.y));

            // Pixel block covering the triangle bounds
            let block_start_x = i::clamp(f::floor_to_int(min_x), 0, (render_target.width() - 1) as i32);
            let block_start_y = i::clamp(f::floor_to_int(min_y), 0, (render_target.height() - 1) as i32);
            let block_end_x = i::clamp(f::ceil_to_int(max_x), 0, (render_target.width() - 1) as i32);
            let block_end_y = i::clamp(f::ceil_to_int(max_y), 0, (render_target.height() - 1) as i32);

            let inv_depths = Float3::new(1.0 / r0.depth, 1.0 / r1.depth, 1.0 / r2.depth);
            // let tx = r0.tex_coords * inv_depths.x;
            // let ty = r1.tex_coords * inv_depths.y;
            // let tz = r2.tex_coords * inv_depths.z;
            // let nx = r0.normals * inv_depths.x;
            // let ny = r1.normals * inv_depths.y;
            // let nz = r2.normals * inv_depths.z;

            // Loop over the block of pixels covering the triangle bounds
            for y in block_start_y..=block_end_y {
                for x in block_start_x..=block_end_x {
                    let p = Float2::new(x as f32, y as f32);
                    let mut weight_a = 0.0;
                    let mut weight_b = 0.0;
                    let mut weight_c = 0.0;

                    if f::point_in_triangle(a, b, c, p, &mut weight_a, &mut weight_b, &mut weight_c) {
                        // Interpolate depths at each vertex to get value for current pixel
                        let depth = 1.0 / (inv_depths.x * weight_a + inv_depths.y * weight_b + inv_depths.z * weight_c);

                        // Depth test (skip if something nearer has already been drawn)
                        if depth >= render_target.get_pixel_depth(x as u32, y as u32) {
                            continue;
                        }

                        // Interpolate texture coordinates at each vertex
                        // let tex_coord = (tx * weight_a + ty * weight_b + tz * weight_c) * depth;
                        // let normal = (nx * weight_a + ny * weight_b + nz * weight_c) * depth;
                        // let col = model.shader.pixel_colour(p, tex_coord, normal, depth);

                        // let color = Float3::new(1.0 / (depth / 5.0), 1.0 / (depth / 5.0), 1.0 / (depth / 5.0));
                        // let color = Float3::new(1.0, 1.0, 1.0);
                        let color = Float3::new(hash((i + 0) as f32), hash((i + 1) as f32), hash((i + 2) as f32));

                        render_target.set_pixel(x as u32, y as u32, color, depth);
                    }
                }
            }
        }
    }
}

fn hash(x: f32) -> f32 {
    let mut v = x.to_bits();
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb_352d);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846c_a68b);
    v ^= v >> 16;
    (v as f32) / (u32::MAX as f32)
}