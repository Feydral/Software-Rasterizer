use crate::{math::numerics::{float2::Float2, float3::Float3}, rasterizer::{camera::Camera, render_target::RenderTarget}, types::{model::Model, rasterizer_point::RasterizerPoint, transform::Transform}};
use crate::math::mathf as f;
use crate::math::mathi as i;

pub fn render(render_target: &mut RenderTarget, data: &mut Vec<Model>, cam: &Camera) {
    for model in data.iter_mut() {
        process_model(render_target, cam, model);
    }

    for model in data {
        for i in (0..model.rasterizer_points().len()).step_by(3) {
            let r0 = &model.rasterizer_points()[i + 0];
            let r1 = &model.rasterizer_points()[i + 1];
            let r2 = &model.rasterizer_points()[i + 2];

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


// Create list of rasterization points for rendering the given model
fn process_model(render_target: &RenderTarget, cam: &Camera, model: &mut Model) {
    let mut view_points: [Float3; 3] = [Float3::ZERO, Float3::ZERO, Float3::ZERO];
    model.rasterizer_points_mut().clear();

    let mut i = 0;
    while i < model.mesh().vertices.len() {
        view_points[0] = vertex_to_view(cam, model.mesh().vertices[i + 0], &model.transform());
        view_points[1] = vertex_to_view(cam, model.mesh().vertices[i + 1], &model.transform());
        view_points[2] = vertex_to_view(cam, model.mesh().vertices[i + 2], &model.transform());

        // Dividing by depths too close to zero causes numerical issues,
        // so use some small positive value for the depth clip threshold
        const NEAR_CLIP_DST: f32 = 0.01;
        let clip0 = view_points[0].z <= NEAR_CLIP_DST;
        let clip1 = view_points[1].z <= NEAR_CLIP_DST;
        let clip2 = view_points[2].z <= NEAR_CLIP_DST;
        let clip_count = i::bool_to_int(clip0) + i::bool_to_int(clip1) + i::bool_to_int(clip2);

        match clip_count {
            0 => {
                add_rasterizer_point(render_target, cam, model, view_points[0], i + 0);
                add_rasterizer_point(render_target, cam, model, view_points[1], i + 1);
                add_rasterizer_point(render_target, cam, model, view_points[2], i + 2);
            }
            1 => {
                // Figure out which point is to be clipped, and the two that will remain
                let index_clip = if clip0 {
                    0
                } else if clip1 {
                    1
                } else {
                    2
                };
                let index_next = (index_clip + 1) % 3;
                let index_prev = (index_clip + 2) % 3;

                let point_clipped = view_points[index_clip];
                let point_a = view_points[index_next];
                let point_b = view_points[index_prev];

                // Fraction along triangle edge at which the depth is equal to the clip distance
                let frac_a = (NEAR_CLIP_DST - point_clipped.z) / (point_a.z - point_clipped.z);
                let frac_b = (NEAR_CLIP_DST - point_clipped.z) / (point_b.z - point_clipped.z);

                // New triangle points (in view space)
                let clip_point_along_edge_a = f::lerp_float3(point_clipped, point_a, frac_a);
                let clip_point_along_edge_b = f::lerp_float3(point_clipped, point_b, frac_b);

                // Create new triangles
                add_rasterizer_point_lerp(render_target,
                    cam,
                    model,
                    clip_point_along_edge_b,
                    i + index_clip,
                    i + index_prev,
                    frac_b,
                );
                add_rasterizer_point_lerp(render_target,
                    cam,
                    model,
                    clip_point_along_edge_a,
                    i + index_clip,
                    i + index_next,
                    frac_a,
                );
                add_rasterizer_point(render_target,cam, model, point_b, i + index_prev);

                add_rasterizer_point_lerp(render_target,
                    cam,
                    model,
                    clip_point_along_edge_a,
                    i + index_clip,
                    i + index_next,
                    frac_a,
                );
                add_rasterizer_point(render_target,cam, model, point_a, i + index_next);
                add_rasterizer_point(render_target,cam, model, point_b, i + index_prev);
            }
            2 => {
                // Figure out which point will not be clipped, and the two that will be
                let index_non_clip = if !clip0 {
                    0
                } else if !clip1 {
                    1
                } else {
                    2
                };
                let index_next = (index_non_clip + 1) % 3;
                let index_prev = (index_non_clip + 2) % 3;

                let point_not_clipped = view_points[index_non_clip];
                let point_a = view_points[index_next];
                let point_b = view_points[index_prev];

                // Fraction along triangle edge at which the depth is equal to the clip distance
                let frac_a =
                    (NEAR_CLIP_DST - point_not_clipped.z) / (point_a.z - point_not_clipped.z);
                let frac_b =
                    (NEAR_CLIP_DST - point_not_clipped.z) / (point_b.z - point_not_clipped.z);

                // New triangle points (in view space)
                let clip_point_along_edge_a = f::lerp_float3(point_not_clipped, point_a, frac_a);
                let clip_point_along_edge_b = f::lerp_float3(point_not_clipped, point_b, frac_b);

                // Create new triangle
                add_rasterizer_point_lerp(render_target, 
                    cam,
                    model,
                    clip_point_along_edge_b,
                    i + index_non_clip,
                    i + index_prev,
                    frac_b,
                );
                add_rasterizer_point(render_target, cam, model, point_not_clipped, i + index_non_clip);
                add_rasterizer_point_lerp(render_target, 
                    cam,
                    model,
                    clip_point_along_edge_a,
                    i + index_non_clip,
                    i + index_next,
                    frac_a,
                );
            }
            _ => {}
        }

        i += 3;
    }
}

fn add_rasterizer_point(render_target: &RenderTarget, cam: &Camera, model: &mut Model, view: Float3, _vertex_index: usize) {
    model.rasterizer_points_mut().push(RasterizerPoint::new(view.z, view_to_screen(render_target, cam, view)));
}

fn add_rasterizer_point_lerp(render_target: &RenderTarget, cam: &Camera, model: &mut Model, view: Float3, _vert_index_a: usize, _vert_index_b: usize, _t: f32) {
    model.rasterizer_points_mut().push(RasterizerPoint::new(view.z, view_to_screen(render_target, cam, view)));
}

#[inline(always)]
fn vertex_to_view(cam: &Camera, vert: Float3, transform: &Transform) -> Float3 {
    let vertex_world = transform.to_world_point(vert);
    let vertex_view = cam.transform.to_local_point(vertex_world);
    vertex_view
}

#[inline(always)]
fn view_to_screen(render_target: &RenderTarget, cam: &Camera, view: Float3) -> Float2 {
    let screen_height_world = (cam.fov_degrees.to_radians() / 2.0).tan() * 2.0;
    let pixels_per_world_unit = render_target.height() as f32 / screen_height_world / view.z;

    let pixel_offset = Float2::new(view.x, view.y) * pixels_per_world_unit;
    Float2::new(render_target.width() as f32, render_target.height() as f32) / 2.0 + pixel_offset
}
