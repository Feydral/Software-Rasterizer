use crate::math::{mathf, mathi};
use crate::math::numerics::{float2::Float2, float3::Float3};
use crate::rasterizer::render_target::RenderTarget;
use crate::types::model::Model;
use crate::types::transform::Transform;

pub fn render(render_target: &mut RenderTarget, scene_data: &Vec<Model>) {
    use mathf as f;
    use mathi as i;

    for model in scene_data {
        for i in (0..model.mesh().vertices.len()).step_by(3) {
            let fov: f32 = 60.0;

            let a = vertex_to_screen(model.mesh().vertices[i + 0], &model.tranform(), &render_target, fov.to_radians());
            let b = vertex_to_screen(model.mesh().vertices[i + 1], &model.tranform(), &render_target, fov.to_radians());
            let c = vertex_to_screen(model.mesh().vertices[i + 2], &model.tranform(), &render_target, fov.to_radians());

            // println!("A: {},{}", a.x, a.y);
            // println!("B: {},{}", b.x, b.y);
            // println!("C: {},{}", c.x, c.y);

            let min_x = f::min(f::min(a.x, b.x), c.x);
            let min_y = f::min(f::min(a.y, b.y), c.y);
            let max_x = f::max(f::max(a.x, b.x), c.x);
            let max_y = f::max(f::max(a.y, b.y), c.y);

            // println!("Min X: {}", min_x);
            // println!("Min Y: {}", max_y);
            // println!("Max X: {}", min_x);
            // println!("Max Y: {}", max_y);

            let block_start_x = i::clamp(f::floor_to_int(min_x), 0, render_target.width() as i32 - 1);
            let block_start_y = i::clamp(f::floor_to_int(min_y), 0, render_target.height() as i32 - 1);
            let block_end_x = i::clamp(f::ceil_to_int(max_x), 0, render_target.width() as i32 - 1);
            let block_end_y = i::clamp(f::ceil_to_int(max_y), 0, render_target.height() as i32 - 1);

            // println!("Triangle BB: ({},{})->({},{})", block_start_x, block_start_y, block_end_x, block_end_y);

            for x in block_start_x..=block_end_x {
                for y in block_start_y..=block_end_y {
                    if !f::point_in_triangle(a, b, c, Float2::new(x as f32, y as f32)) {
                        continue;
                    }
                    
                    render_target.set_pixel(x as u32, y as u32, Float3::new(hash((i + 0) as f32), hash((i + 1) as f32), hash((i + 2) as f32)), 0.0);
                }
            }
        }
    }
}

fn vertex_to_screen(vertex: Float3, transform: &Transform, render_target: &RenderTarget, fov: f32) -> Float2 {
    let world_vertex = transform.to_world_point(vertex);

    let screen_height_world = (fov / 2.0).tan() * 2.0;
    let pixels_per_world_unit = render_target.height() as f32 / screen_height_world / world_vertex.z;

    let pixel_offset = Float2::new(world_vertex.x, world_vertex.y) * pixels_per_world_unit;
    let v_screen_pos = Float2::new(render_target.width() as f32, render_target.height() as f32) / 2.0 + pixel_offset;

    // println!("{}, {}", v_screen_pos.x, v_screen_pos.y);
    v_screen_pos
}

fn hash(x: f32) -> f32 {
    // 1. Float-Bits holen (deterministisch)
    let mut v = x.to_bits();

    // 2. sehr simple Integer-Mischung
    v ^= v >> 16;
    v = v.wrapping_mul(0x7feb_352d);
    v ^= v >> 15;
    v = v.wrapping_mul(0x846c_a68b);
    v ^= v >> 16;

    // 3. auf [0, 1) skalieren
    (v as f32) / (u32::MAX as f32)
}
