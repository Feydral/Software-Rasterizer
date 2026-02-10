use crate::math::numerics::{float2::Float2, float3::Float3};

// Test if point p is inside triangle ABC
// Note: non-clockwise triangles are considered 'back-faces' and are ignored
#[inline(always)]
#[allow(dead_code)]
pub fn point_in_triangle(a: Float2, b: Float2, c: Float2, p: Float2, weight_a: &mut f32, weight_b: &mut f32, weight_c: &mut f32) -> bool {
    // Test if point is on right side of each edge segment
    let area_abp = signed_parallelogram_area(a, b, p);
    let area_bcp = signed_parallelogram_area(b, c, p);
    let area_cap = signed_parallelogram_area(c, a, p);
    let in_tri = area_abp >= 0.0 && area_bcp >= 0.0 && area_cap >= 0.0;

    // Weighting factors (barycentric coordinates)
    let total_area = area_abp + area_bcp + area_cap;
    let inv_area_sum = 1.0 / total_area;

    *weight_a = area_bcp * inv_area_sum;
    *weight_b = area_cap * inv_area_sum;
    *weight_c = area_abp * inv_area_sum;

    in_tri && total_area > 0.0
}

#[inline(always)]
pub fn signed_parallelogram_area(a: Float2, b: Float2, c: Float2) -> f32 {
    (c.x - a.x) * (b.y - a.y) + (c.y - a.y) * (a.x - b.x)
}

#[inline(always)]
#[allow(dead_code)]
pub fn round_to_int(value: f32) -> i32 {
    value.round() as i32
}

#[inline(always)]
#[allow(dead_code)]
pub fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}

#[inline(always)]
#[allow(dead_code)]
pub fn max(a: f32, b: f32) -> f32 {
    a.max(b)
}

#[inline(always)]
#[allow(dead_code)]
pub fn clamp(a: f32, min: f32, max: f32) -> f32 {
    a.clamp(min, max)
}

#[inline(always)]
#[allow(dead_code)]
pub fn ceil_to_int(a: f32) -> i32 {
    a.ceil() as i32
}

#[inline(always)]
#[allow(dead_code)]
pub fn floor_to_int(a: f32) -> i32 {
    a.floor() as i32
}

#[inline(always)]
#[allow(dead_code)]
pub fn lerp_float3(a: Float3, b: Float3, t: f32) -> Float3 {
	a + (b - a) * t.clamp(0.0, 1.0)
}

#[inline(always)]
#[allow(dead_code)]
pub fn lerp_float2(a: Float2, b: Float2, t: f32) -> Float2 {
	a + (b - a) * t.clamp(0.0, 1.0)
}


#[inline(always)]
#[allow(dead_code)]
pub fn transform_vector(ihat: Float3, jhat: Float3, khat: Float3, v: Float3) -> Float3 {
    ihat * v.x + jhat * v.y + khat * v.z
}