use crate::types::transform::Transform;

pub struct Camera {
    pub fov_degrees: f32,
    pub transform: Transform,
}

impl Camera {
    pub fn new(fov_degrees: f32) -> Self {
        Self {
            fov_degrees,
            transform: Transform::default(),
        }
    }
}