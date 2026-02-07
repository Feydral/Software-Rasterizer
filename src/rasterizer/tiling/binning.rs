use crate::math::numerics::uint2::UInt2;
use crate::types::rasterizer_model::RasterizerModel;
use crate::math::mathf as f;

pub fn bin_triangles_for_tile(model: &RasterizerModel, tile_min: UInt2, tile_max: UInt2) -> Vec<u32> {
    let mut result = Vec::new();

    let tile_min_x = tile_min.x as f32;
    let tile_min_y = tile_min.y as f32;
    let tile_max_x = tile_max.x as f32;
    let tile_max_y = tile_max.y as f32;

    let points = model.points.as_slice();

    let mut triangle_index = 0;

    for i in (0..points.len()).step_by(3) {
        let a = points[i + 0].screen_pos;
        let b = points[i + 1].screen_pos;
        let c = points[i + 2].screen_pos;

        let min_x = f::min(a.x, f::min(b.x, c.x));
        let min_y = f::min(a.y, f::min(b.y, c.y));
        let max_x = f::max(a.x, f::max(b.x, c.x));
        let max_y = f::max(a.y, f::max(b.y, c.y));

        if max_x >= tile_min_x && max_y >= tile_min_y && min_x <= tile_max_x && min_y <= tile_max_y {
            result.push(triangle_index);
        }

        triangle_index += 1;
    }

    result
}
