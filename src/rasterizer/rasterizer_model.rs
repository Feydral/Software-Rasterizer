use crate::math::numerics::float2::Float2;
use crate::math::numerics::float3::Float3;
use crate::rasterizer::camera::Camera;
use crate::rasterizer::rasterizer_point::RasterizerPoint;
use crate::rasterizer::render_target::RenderTarget;
use crate::types::model::Model;
use crate::types::transform::Transform;
use crate::math::mathf as f;
use crate::math::mathi as i;

pub struct RasterizerModel {
    pub points: Vec<RasterizerPoint>,
}

impl RasterizerModel {
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

// Create list of rasterization points for rendering the given model
    pub fn process_model(model: &Model, render_target: &RenderTarget, cam: &Camera) -> Self {
        let mut view_points: [Float3; 3] = [Float3::ZERO, Float3::ZERO, Float3::ZERO];
        let mut r_model = Self::new();

        for i in (0..model.mesh.indices.len()).step_by(3) {
        let idx0 = model.mesh.indices[i + 0] as usize;
        let idx1 = model.mesh.indices[i + 1] as usize;
        let idx2 = model.mesh.indices[i + 2] as usize;

        view_points[0] = Self::vertex_to_view(cam, model.mesh.vertices[idx0], &model.transform);
        view_points[1] = Self::vertex_to_view(cam, model.mesh.vertices[idx1], &model.transform);
        view_points[2] = Self::vertex_to_view(cam, model.mesh.vertices[idx2], &model.transform);

        // Dividing by depths too close to zero causes numerical issues,
        // so use some small positive value for the depth clip threshold
        const NEAR_CLIP_DST: f32 = 0.01;
        let clip0 = view_points[0].z <= NEAR_CLIP_DST;
        let clip1 = view_points[1].z <= NEAR_CLIP_DST;
        let clip2 = view_points[2].z <= NEAR_CLIP_DST;
        let clip_count = i::bool_to_int(clip0) + i::bool_to_int(clip1) + i::bool_to_int(clip2);

        match clip_count {
            0 => {
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, view_points[0], idx0);
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, view_points[1], idx1);
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, view_points[2], idx2);
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
                Self::add_rasterizer_point_lerp(&mut r_model.points, render_target, cam, clip_point_along_edge_b, i + index_clip, i + index_prev, frac_b);
                Self::add_rasterizer_point_lerp(&mut r_model.points, render_target, cam, clip_point_along_edge_a, i + index_clip, i + index_next, frac_a);
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, point_b, i + index_prev);

                Self::add_rasterizer_point_lerp(&mut r_model.points, render_target, cam, clip_point_along_edge_a, i + index_clip, i + index_next, frac_a);
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, point_a, i + index_next);
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, point_b, i + index_prev);
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
                let frac_a = (NEAR_CLIP_DST - point_not_clipped.z) / (point_a.z - point_not_clipped.z);
                let frac_b = (NEAR_CLIP_DST - point_not_clipped.z) / (point_b.z - point_not_clipped.z);

                // New triangle points (in view space)
                let clip_point_along_edge_a = f::lerp_float3(point_not_clipped, point_a, frac_a);
                let clip_point_along_edge_b = f::lerp_float3(point_not_clipped, point_b, frac_b);

                // Create new triangle
                Self::add_rasterizer_point_lerp(&mut r_model.points, render_target, cam, clip_point_along_edge_b, i + index_non_clip, i + index_prev, frac_b, );
                Self::add_rasterizer_point(&mut r_model.points, render_target, cam, point_not_clipped, i + index_non_clip);
                Self::add_rasterizer_point_lerp(&mut r_model.points, render_target, cam, clip_point_along_edge_a, i + index_non_clip, i + index_next, frac_a, );
            }
            _ => {}
        }
    }

    r_model
}

    fn add_rasterizer_point(points: &mut Vec<RasterizerPoint>, render_target: &RenderTarget, cam: &Camera, view: Float3, _vertex_index: usize) {
        points.push(RasterizerPoint::new(view.z, Self::view_to_screen(render_target, cam, view)));
    }

    fn add_rasterizer_point_lerp(points: &mut Vec<RasterizerPoint>, render_target: &RenderTarget, cam: &Camera, view: Float3, _vert_index_a: usize, _vert_index_b: usize, _t: f32) {
        points.push(RasterizerPoint::new(view.z, Self::view_to_screen(render_target, cam, view)));
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
}