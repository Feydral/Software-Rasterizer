use crate::math::numerics::float3::Float3;

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
    pub fn set_position(&mut self, position: Float3) {
        self.set_pos_rot_scale(position, self.rotation, self.scale);
    }

    pub fn translate(&mut self, delta_postion: Float3) {
        self.set_pos_rot_scale(self.position + delta_postion, self.rotation, self.scale);
    }

    pub fn set_rotation(&mut self, rotation: Float3) {
        self.set_pos_rot_scale(self.position, rotation, self.scale);
    }

    pub fn rotate(&mut self, delta_rotation: Float3) {
        self.set_pos_rot_scale(self.position, self.rotation + delta_rotation, self.scale);
    }

    pub fn set_scale(&mut self, scale: Float3) {
        self.set_pos_rot_scale(self.position, self.rotation, scale);
    }

    pub fn scale(&mut self, delta_scale: Float3) {
        self.set_pos_rot_scale(self.position, self.rotation, self.scale + delta_scale);
    }

    pub fn set_pos_rot_scale(&mut self, pos: Float3, rot: Float3, scale: Float3) {
        self.position = pos;
        self.scale = scale;
        self.rotation = rot;
        self.update_basis_vectors();
    }
    
    // ---------------- World / Local ----------------
    pub fn to_world_point(&self, local: Float3) -> Float3 {
        let mut p = Self::transform_vector(
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

    pub fn to_local_point(&self, world: Float3) -> Float3 {
        let mut p = if let Some(parent) = &self.parent {
            parent.to_local_point(world)
        } else {
            world
        };

        p = Self::transform_vector(
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

    fn update_basis_vectors(&mut self) {
        let (r, u, f) = self.get_basis_vectors();
        self.right = r;
        self.up = u;
        self.forward = f;

        let (r_inv, u_inv, f_inv) = self.get_inverse_basis_vectors();
        self.right_inv = r_inv;
        self.up_inv = u_inv;
        self.forward_inv = f_inv;
    }

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
        let ihat_py = Self::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, ihat_pitch);
        let jhat_py = Self::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, jhat_pitch);
        let khat_py = Self::transform_vector(ihat_yaw, jhat_yaw, khat_yaw, khat_pitch);

        // + Roll
        let ihat = Self::transform_vector(ihat_py, jhat_py, khat_py, ihat_roll);
        let jhat = Self::transform_vector(ihat_py, jhat_py, khat_py, jhat_roll);
        let khat = Self::transform_vector(ihat_py, jhat_py, khat_py, khat_roll);

        (ihat, jhat, khat)
    }

    fn get_inverse_basis_vectors(&self) -> (Float3, Float3, Float3) {
        let (r, u, f) = self.get_basis_vectors();
        let r_inv = Float3::new(r.x, u.x, f.x);
        let u_inv = Float3::new(r.y, u.y, f.y);
        let f_inv = Float3::new(r.z, u.z, f.z);
        (r_inv, u_inv, f_inv)
    }

    #[inline(always)]
    fn transform_vector(ihat: Float3, jhat: Float3, khat: Float3, v: Float3) -> Float3 {
        ihat * v.x + jhat * v.y + khat * v.z
    }
}
