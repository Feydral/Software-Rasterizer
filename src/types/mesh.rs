#![allow(dead_code)]

use crate::math::numerics::{float2::Float2, float3::Float3};

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Float3>,
    pub indices: Vec<u32>,
    pub normals: Vec<Float3>,
    pub uvs: Vec<Float2>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
        }
    }
}