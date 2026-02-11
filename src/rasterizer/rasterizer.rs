use crate::{math::numerics::{float2::Float2, float3::Float3, float4::Float4}, rasterizer::{camera::Camera, rasterizer_point::RasterizerPoint, render_target::RenderTarget}, types::{model::Model, transform::Transform}};
use crate::math::mathf as f;
use crate::math::mathi as i;

pub fn render(render_target: &mut RenderTarget, models: &mut Vec<Model>, cam: &Camera) {
    for model in models.iter_mut() {
        process_model(model, render_target, cam);
    }

    for model in models.iter_mut() {
        for i in (0..model.rasterizer_points.len()).step_by(3) {
            let r0 = &model.rasterizer_points[i + 0];
            let r1 = &model.rasterizer_points[i + 1];
            let r2 = &model.rasterizer_points[i + 2];

            if model.shader.wire_frame() {
                let line_color = Float4::new(1.0, 1.0, 1.0, 1.0);
                draw_line(render_target, r0, r1, line_color);
                draw_line(render_target, r1, r2, line_color);
                draw_line(render_target, r2, r0, line_color);
                continue;
            }

            let a = r0.screen_pos;
            let b = r1.screen_pos;
            let c = r2.screen_pos;

            let min_x = f::min(a.x, f::min(b.x, c.x));
            let min_y = f::min(a.y, f::min(b.y, c.y));
            let max_x = f::max(a.x, f::max(b.x, c.x));
            let max_y = f::max(a.y, f::max(b.y, c.y));

            let block_start_x = i::clamp(f::floor_to_int(min_x), 0, (render_target.width() - 1) as i32);
            let block_start_y = i::clamp(f::floor_to_int(min_y), 0, (render_target.height() - 1) as i32);
            let block_end_x = i::clamp(f::ceil_to_int(max_x), 0, (render_target.width() - 1) as i32);
            let block_end_y = i::clamp(f::ceil_to_int(max_y), 0, (render_target.height() - 1) as i32);

            let inv_depths = Float3::new(1.0 / r0.depth, 1.0 / r1.depth, 1.0 / r2.depth);
            let tx = r0.tex_coords * inv_depths.x;
            let ty = r1.tex_coords * inv_depths.y;
            let tz = r2.tex_coords * inv_depths.z;
            let nx = r0.normals * inv_depths.x;
            let ny = r1.normals * inv_depths.y;
            let nz = r2.normals * inv_depths.z;

            for y in block_start_y..=block_end_y {
                for x in block_start_x..=block_end_x {
                    let p = Float2::new(x as f32 + 0.5, y as f32 + 0.5);
                    let mut weight_a = 0.0;
                    let mut weight_b = 0.0;
                    let mut weight_c = 0.0;

                    if f::point_in_triangle(a, b, c, p, &mut weight_a, &mut weight_b, &mut weight_c) {
                        let depth = 1.0 / (inv_depths.x * weight_a + inv_depths.y * weight_b + inv_depths.z * weight_c);

                        if depth >= render_target.get_pixel_depth(x as u32, y as u32) {
                            continue;
                        }

                        let uv = (tx * weight_a + ty * weight_b + tz * weight_c) * depth;
                        let normal = (nx * weight_a + ny * weight_b + nz * weight_c) * depth;

                        let color = model.shader.pixel_color(p, uv, normal, depth);

                        render_target.set_pixel(x as u32, y as u32, color, depth);
                    }
                }
            }
        }
    }
}

pub fn process_model(model: &mut Model, render_target: &RenderTarget, cam: &Camera) {
    let mut view_points: [Float3; 3] = [Float3::ZERO, Float3::ZERO, Float3::ZERO];
    model.rasterizer_points.clear();

    for i in (0..model.mesh.indices.len()).step_by(3) {
        let idx0 = model.mesh.indices[i + 0] as usize;
        let idx1 = model.mesh.indices[i + 1] as usize;
        let idx2 = model.mesh.indices[i + 2] as usize;

        view_points[0] = vertex_to_view(cam, model.mesh.vertices[idx0], &model.transform);
        view_points[1] = vertex_to_view(cam, model.mesh.vertices[idx1], &model.transform);
        view_points[2] = vertex_to_view(cam, model.mesh.vertices[idx2], &model.transform);

        const NEAR_CLIP_DST: f32 = 0.01;
        let clip0 = view_points[0].z <= NEAR_CLIP_DST;
        let clip1 = view_points[1].z <= NEAR_CLIP_DST;
        let clip2 = view_points[2].z <= NEAR_CLIP_DST;
        let clip_count = i::bool_to_int(clip0) + i::bool_to_int(clip1) + i::bool_to_int(clip2);

        match clip_count {
            0 => {
                add_rasterizer_point(model, render_target, cam, view_points[0], idx0);
                add_rasterizer_point(model, render_target, cam, view_points[1], idx1);
                add_rasterizer_point(model, render_target, cam, view_points[2], idx2);
            }
            1 => {
                let index_clip = if clip0 { 0 } else if clip1 { 1 } else { 2 };
                let index_next = (index_clip + 1) % 3;
                let index_prev = (index_clip + 2) % 3;

                let point_clipped = view_points[index_clip];
                let point_a = view_points[index_next];
                let point_b = view_points[index_prev];

                let frac_a = (NEAR_CLIP_DST - point_clipped.z) / (point_a.z - point_clipped.z);
                let frac_b = (NEAR_CLIP_DST - point_clipped.z) / (point_b.z - point_clipped.z);

                let clip_point_a = f::lerp_float3(point_clipped, point_a, frac_a);
                let clip_point_b = f::lerp_float3(point_clipped, point_b, frac_b);

                let idx_clip = model.mesh.indices[i + index_clip] as usize;
                let idx_next = model.mesh.indices[i + index_next] as usize;
                let idx_prev = model.mesh.indices[i + index_prev] as usize;

                add_rasterizer_point_lerp(model, render_target, cam, clip_point_b, idx_clip, idx_prev, frac_b);
                add_rasterizer_point_lerp(model, render_target, cam, clip_point_a, idx_clip, idx_next, frac_a);
                add_rasterizer_point(model, render_target, cam, point_b, idx_prev);

                add_rasterizer_point_lerp(model, render_target, cam, clip_point_a, idx_clip, idx_next, frac_a);
                add_rasterizer_point(model, render_target, cam, point_a, idx_next);
                add_rasterizer_point(model, render_target, cam, point_b, idx_prev);
            }
            2 => {
                let index_non_clip = if !clip0 { 0 } else if !clip1 { 1 } else { 2 };
                let index_next = (index_non_clip + 1) % 3;
                let index_prev = (index_non_clip + 2) % 3;

                let point_nc = view_points[index_non_clip];
                let point_a = view_points[index_next];
                let point_b = view_points[index_prev];

                let frac_a = (NEAR_CLIP_DST - point_nc.z) / (point_a.z - point_nc.z);
                let frac_b = (NEAR_CLIP_DST - point_nc.z) / (point_b.z - point_nc.z);

                let clip_point_a = f::lerp_float3(point_nc, point_a, frac_a);
                let clip_point_b = f::lerp_float3(point_nc, point_b, frac_b);

                let idx_nc = model.mesh.indices[i + index_non_clip] as usize;
                let idx_next = model.mesh.indices[i + index_next] as usize;
                let idx_prev = model.mesh.indices[i + index_prev] as usize;

                add_rasterizer_point_lerp(model, render_target, cam, clip_point_b, idx_nc, idx_prev, frac_b);
                add_rasterizer_point(model, render_target, cam, point_nc, idx_nc);
                add_rasterizer_point_lerp(model, render_target, cam, clip_point_a, idx_nc, idx_next, frac_a);
            }
            _ => {}
        }
    }
}

fn add_rasterizer_point(model: &mut Model, render_target: &RenderTarget, cam: &Camera, view: Float3, vertex_idx: usize) {
    let normal_world = normal_to_normalview(&model.transform, model.mesh.normals[vertex_idx]);
    let normal_view = normalview_to_screen(cam, normal_world);

    model.rasterizer_points.push(RasterizerPoint::new(
        view.z,
        view_to_screen(render_target, cam, view),
        model.mesh.uvs[vertex_idx],
        normal_view,
    ));
}

fn add_rasterizer_point_lerp(model: &mut Model, render_target: &RenderTarget, cam: &Camera, view: Float3, vertex_idx_a: usize, vertex_idx_b: usize, t: f32) {
    let normal_a_world = normal_to_normalview(&model.transform, model.mesh.normals[vertex_idx_a]);
    let normal_b_world = normal_to_normalview(&model.transform, model.mesh.normals[vertex_idx_b]);

    let normal_world = f::lerp_float3(normal_a_world, normal_b_world, t).normalize();

    let normal_view = normalview_to_screen(cam, normal_world);

    model.rasterizer_points.push(RasterizerPoint::new(
        view.z,
        view_to_screen(render_target, cam, view),
        f::lerp_float2(model.mesh.uvs[vertex_idx_a], model.mesh.uvs[vertex_idx_b], t),
        normal_view,
    ));
}

fn normalview_to_screen(cam: &Camera, normal_world: Float3) -> Float3 {
    cam.transform.to_local_vector(normal_world).normalize()
}

#[inline(always)]
fn normal_to_normalview(transform: &Transform, normal_local: Float3) -> Float3 {
    transform.transform_vector_along_self(normal_local)
}

#[inline(always)]
fn vertex_to_view(cam: &Camera, vertex: Float3, transform: &Transform) -> Float3 {
    let vertex_world = transform.to_world_point(vertex);
    cam.transform.to_local_point(vertex_world)
}

#[inline(always)]
fn view_to_screen(render_target: &RenderTarget, cam: &Camera, view: Float3) -> Float2 {
    let screen_height_world = (cam.fov_degrees.to_radians() / 2.0).tan() * 2.0;
    let pixels_per_world_unit = render_target.height() as f32 / screen_height_world / view.z;

    let pixel_offset = Float2::new(view.x, view.y) * pixels_per_world_unit;
    Float2::new(render_target.width() as f32, render_target.height() as f32) / 2.0 + pixel_offset
}

fn draw_line(render_target: &mut RenderTarget, start: &RasterizerPoint, end: &RasterizerPoint, color: Float4) {
    let x0 = start.screen_pos.x.round() as i32;
    let y0 = start.screen_pos.y.round() as i32;
    let x1 = end.screen_pos.x.round() as i32;
    let y1 = end.screen_pos.y.round() as i32;

    let z0 = start.depth;
    let z1 = end.depth;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let steps = if dx > dy { dx } else { dy };
    let steps = if steps == 0 { 1 } else { steps } as i32;

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;

    let max_iters = (dx + dy + 1000) as usize;
    let mut iter_count: usize = 0;
    let mut step: i32 = 0;

    loop {
        iter_count += 1;
        if iter_count > max_iters {
            break;
        }

        if x >= 0 && y >= 0 && (x as u32) < render_target.width() && (y as u32) < render_target.height() {
            let t = (step as f32) / (steps as f32);
            let depth = z0 * (1.0 - t) + z1 * t;

            let ux = x as u32;
            let uy = y as u32;
            if depth < render_target.get_pixel_depth(ux, uy) {
                render_target.set_pixel(ux, uy, color, depth);
            }
        }

        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }

        step = step.saturating_add(1);
    }
}