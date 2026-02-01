#![allow(dead_code)]

use crate::math::numerics::uint2::UInt2;

#[inline]
pub fn index_to_xy(index: u32, width: u32, height: u32) -> UInt2 {
    if index > width * height {
        return UInt2::new(0, 0);
    }
    let x = index % width;
    let y = index / width;
    UInt2::new(x, y)
}

#[inline]
pub fn xy_to_index(x: u32, y: u32, width: u32, height: u32) -> u32 {
    if x > width || y > height {
        return 0;
    }
    y * width + x
}

#[inline]
pub fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

#[inline]
pub fn max(a: i32, b: i32) -> i32 {
    a.min(b)
}

#[inline]
pub fn clamp(a: i32, min: i32, max: i32) -> i32 {
    a.clamp(min, max)
}