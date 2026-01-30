use crate::math::numerics::float2::Float2;

#[inline]
pub fn point_in_triangle(a: Float2, b: Float2, c: Float2, p: Float2) -> bool {
    let side_ab = point_on_right_side_of_line(a, b, p);
    let side_bc = point_on_right_side_of_line(b, c, p);
    let side_ca = point_on_right_side_of_line(c, a, p);
    side_ab == side_bc && side_bc == side_ca
}

#[inline]
fn point_on_right_side_of_line(a: Float2, b: Float2, p: Float2) -> bool {
    let ap = p - a;
    let ap_perpendicular = perpendicular(b - a);
    ap.dot(ap_perpendicular) >= 0.0
}

#[inline]
fn perpendicular(vec: Float2) -> Float2 {
    Float2::new(vec.y, -vec.x)
}

#[inline]
pub fn min(a: f32, b: f32) -> f32 {
    a.min(b)
}

#[inline]
pub fn max(a: f32, b: f32) -> f32 {
    a.max(b)
}

#[inline]
pub fn clamp(a: f32, min: f32, max: f32) -> f32 {
    a.clamp(min, max)
}

#[inline]
pub fn ceil_to_int(a: f32) -> i32 {
    a.ceil() as i32
}

#[inline]
pub fn floor_to_int(a: f32) -> i32 {
    a.floor() as i32
}