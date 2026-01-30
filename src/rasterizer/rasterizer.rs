use crate::math::{mathf, mathi};
use crate::math::numerics::float2::Float2;
use crate::math::numerics::float3::Float3;
use crate::types::mesh::Mesh;
use crate::types::scene::Scene;
use crate::rasterizer::render_target::RenderTarget;

pub fn render(render_target: &mut RenderTarget, scene: &Vec<Mesh>) {
    use mathf as f;
    use mathi as i;

    for mesh in scene {
        for i in (0..mesh.vertices.len()).step_by(3) {
            let a = Float2::new(mesh.vertices[i + 0].x, mesh.vertices[i + 0].y);
            let b = Float2::new(mesh.vertices[i + 1].x, mesh.vertices[i + 1].y);
            let c = Float2::new(mesh.vertices[i + 2].x, mesh.vertices[i + 2].y);

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
                    if !f::point_in_triangle(a, b, c, Float2::new(x as f32 + 0.5, y as f32 + 0.5)) {
                        continue;
                    }
                    
                    render_target.set_pixel(x as u32, y as u32, Float3::new(1.0, 1.0, 1.0), 0.0);
                }
            }
        }
    }
}