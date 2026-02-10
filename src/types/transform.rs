use crate::math::{mathf, numerics::float3::Float3};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Float3,
    pub rotation: Float3, // pitch=x, yaw=y, roll=z
    pub scale: Float3,
    pub parent: Option<Box<Transform>>,

    right: Float3,
    up: Float3,
    forward: Float3,

    right_inv: Float3,
    up_inv: Float3,
    forward_inv: Float3,
}

impl Default for Transform {
    fn default() -> Self {
        let mut t = Self {
            position: Float3::new(0.0, 0.0, 0.0),
            rotation: Float3::new(0.0, 0.0, 0.0),
            scale: Float3::new(1.0, 1.0, 1.0),
            parent: None,
            right: Float3::new(1.0, 0.0, 0.0),
            up: Float3::new(0.0, 1.0, 0.0),
            forward: Float3::new(0.0, 0.0, 1.0),
            right_inv: Float3::new(1.0, 0.0, 0.0),
            up_inv: Float3::new(0.0, 1.0, 0.0),
            forward_inv: Float3::new(0.0, 0.0, 1.0),
        };
        t.update_basis_vectors();
        t
    }
}

impl Transform {
    #[allow(dead_code)]
    pub fn set_position(&mut self, position: Float3) {
        self.set_pos_rot_scale(position, self.rotation, self.scale);
    }

    #[allow(dead_code)]
    pub fn translate(&mut self, delta_postion: Float3) {
        self.set_pos_rot_scale(self.position + delta_postion, self.rotation, self.scale);
    }

    #[allow(dead_code)]
    pub fn set_rotation(&mut self, rotation: Float3) {
        self.set_pos_rot_scale(self.position, rotation, self.scale);
    }

    #[allow(dead_code)]
    pub fn rotate(&mut self, delta_rotation: Float3) {
        self.set_pos_rot_scale(self.position, self.rotation + delta_rotation, self.scale);
    }

    #[allow(dead_code)]
    pub fn set_scale(&mut self, scale: Float3) {
        self.set_pos_rot_scale(self.position, self.rotation, scale);
    }

    #[allow(dead_code)]
    pub fn scale(&mut self, delta_scale: Float3) {
        self.set_pos_rot_scale(self.position, self.rotation, self.scale + delta_scale);
    }

    fn set_pos_rot_scale(&mut self, pos: Float3, rot: Float3, scale: Float3) {
        self.position = pos;
        self.scale = scale;
        self.rotation = rot;
        self.update_basis_vectors();
    }
    
    // ---------------- World / Local ----------------
    #[inline(always)]
    #[allow(dead_code)]
    pub fn to_world_point(&self, local: Float3) -> Float3 {
        let mut p = mathf::transform_vector(
            self.right * self.scale.x,
            self.up * self.scale.y,
            self.forward * self.scale.z,
            local,
        ) + self.position;

        if let Some(parent) = &self.parent {
            p = parent.to_world_point(p);
        }
        p
    }
    
    #[inline(always)]
    #[allow(dead_code)]
    pub fn to_local_point(&self, world: Float3) -> Float3 {
        let mut p = if let Some(parent) = &self.parent {
            parent.to_local_point(world)
        } else {
            world
        };

        p = mathf::transform_vector(
            self.right_inv,
            self.up_inv,
            self.forward_inv,
            p - self.position,
        );

        p.x /= self.scale.x;
        p.y /= self.scale.y;
        p.z /= self.scale.z;
        p
    }
    
    #[inline(always)]
    #[allow(dead_code)]
    pub fn to_local_vector(&self, world_vec: Float3) -> Float3 {
        let mut v = if let Some(parent) = &self.parent {
            parent.to_local_vector(world_vec)
        } else {
            world_vec
        };

        v = mathf::transform_vector(self.right_inv, self.up_inv, self.forward_inv, v);
        v.normalize()
    }
    
    #[inline(always)]
    #[allow(dead_code)]
    pub fn to_world_vector(&self, local_vec: Float3) -> Float3 {
        let mut v = mathf::transform_vector(self.right, self.up, self.forward, local_vec);
        if let Some(parent) = &self.parent {
            v = parent.to_world_vector(v);
        }
        v.normalize()
    }
    
    #[inline(always)]
    #[allow(dead_code)]
    fn update_basis_vectors(&mut self) {
        let pitch = self.rotation.x;
        let yaw = self.rotation.y;
        let roll = self.rotation.z;
        
        // Forward aus Pitch/Yaw
        let forward = Float3::new(
            yaw.sin() * pitch.cos(),
            pitch.sin(),
            yaw.cos() * pitch.cos()
        ).normalize();
    
        let world_up = Float3::new(0.0, 1.0, 0.0);
        let mut right = forward.cross(world_up).normalize();
        let mut up = right.cross(forward).normalize();
    
        // Roll um Forward-Achse anwenden
        if roll != 0.0 {
            let cos_r = roll.cos();
            let sin_r = roll.sin();
        
            let new_right = right * cos_r + up * sin_r;
            let new_up = Float3::ZERO - right * sin_r + up * cos_r;
        
            right = new_right;
            up = new_up;
        }
    
        self.forward = forward;
        self.right = right;
        self.up = up;
    
        // Inverse Basisvektoren für to_local_point / to_local_vector
        self.right_inv = Float3::new(right.x, up.x, forward.x);
        self.up_inv = Float3::new(right.y, up.y, forward.y);
        self.forward_inv = Float3::new(right.z, up.z, forward.z);
    }
    
    #[inline(always)]
    #[allow(dead_code)]
    fn get_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let pitch = self.rotation.x;
        let yaw = self.rotation.y;
        let roll = self.rotation.z;

        // Yaw
        let ihat_yaw = Float3::new(yaw.cos(), 0.0, yaw.sin());
        let jhat_yaw = Float3::new(0.0, 1.0, 0.0);
        let khat_yaw = Float3::new(-yaw.sin(), 0.0, yaw.cos());

        // Pitch
        let ihat_pitch = Float3::new(1.0, 0.0, 0.0);
        let jhat_pitch = Float3::new(0.0, pitch.cos(), -pitch.sin());
        let khat_pitch = Float3::new(0.0, pitch.sin(), pitch.cos());

        // Roll
        let ihat_roll = Float3::new(roll.cos(), roll.sin(), 0.0);
        let jhat_roll = Float3::new(-roll.sin(), roll.cos(), 0.0);
        let khat_roll = Float3::new(0.0, 0.0, 1.0);

        // Pitch + Yaw
        let ihat_py = mathf::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, ihat_pitch);
        let jhat_py = mathf::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, jhat_pitch);
        let khat_py = mathf::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, khat_pitch);

        // + Roll
        let ihat = mathf::transform_vector(ihat_py, jhat_py, khat_py, ihat_roll);
        let jhat = mathf::transform_vector(ihat_py, jhat_py, khat_py, jhat_roll);
        let khat = mathf::transform_vector(ihat_py, jhat_py, khat_py, khat_roll);

        (ihat, jhat, khat)
    }
    
    #[inline(always)]
    #[allow(dead_code)]
    fn get_inverse_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let (r, u, f) = self.get_basis_vectors();
        let r_inv = Float3::new(r.x, u.x, f.x);
        let u_inv = Float3::new(r.y, u.y, f.y);
        let f_inv = Float3::new(r.z, u.z, f.z);
        (r_inv, u_inv, f_inv)
    }

    #[inline(always)]
    #[allow(dead_code)]
    pub fn transform_vector_along_self(&self, v: Float3) -> Float3 {
        let (ihat, jhat, khat) = self.get_basis_vectors();
        ihat * v.x + jhat * v.y + khat * v.z
    }

    #[allow(dead_code)]
    pub fn forward(&self) -> Float3 {
        self.forward
    }
    #[allow(dead_code)]
    pub fn backward(&self) -> Float3 {
        Float3::ZERO - self.forward
    }
    #[allow(dead_code)]
    pub fn right(&self) -> Float3 {
        self.right
    }
    #[allow(dead_code)]
    pub fn left(&self) -> Float3 {
        Float3::ZERO - self.right
    }
    #[allow(dead_code)]
    pub fn up(&self) -> Float3 {
        self.up
    }
    #[allow(dead_code)]
    pub fn down(&self) -> Float3 {
        Float3::ZERO - self.up
    }
}
