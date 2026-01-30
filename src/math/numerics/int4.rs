use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

impl Int4 {
    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Int4 {
        Int4 { x, y, z, w }
    }
}

// ======= ADD ======= 
impl Add for Int4 {
    type Output = Int4;
    fn add(self, rhs: Int4) -> Int4 {
        Int4 { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y, 
            z: self.z + rhs.z, 
            w: self.w + rhs.w 
        }
    }
}
impl Add<i32> for Int4 {
    type Output = Int4;
    fn add(self, rhs: i32) -> Int4 {
        Int4 { 
            x: self.x + rhs, 
            y: self.y + rhs, 
            z: self.z + rhs, 
            w: self.w + rhs 
        }
    }
}
impl Add<Int4> for i32 {
    type Output = Int4;
    fn add(self, rhs: Int4) -> Int4 {
        Int4 { 
            x: self + rhs.x, 
            y: self + rhs.y, 
            z: self + rhs.z, 
            w: self + rhs.w 
        }
    }
}

impl AddAssign for Int4 {
    fn add_assign(&mut self, rhs: Int4) {
        self.x += rhs.x; 
        self.y += rhs.y; 
        self.z += rhs.z; 
        self.w += rhs.w;
    }
}
impl AddAssign<i32> for Int4 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; 
        self.y += rhs; 
        self.z += rhs; 
        self.w += rhs;
    }
}

// ======= SUB ======= 
impl Sub for Int4 {
    type Output = Int4;
    fn sub(self, rhs: Int4) -> Int4 {
        Int4 { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y, 
            z: self.z - rhs.z, 
            w: self.w - rhs.w 
        }
    }
}
impl Sub<i32> for Int4 {
    type Output = Int4;
    fn sub(self, rhs: i32) -> Int4 {
        Int4 { 
            x: self.x - rhs, 
            y: self.y - rhs, 
            z: self.z - rhs, 
            w: self.w - rhs 
        }
    }
}
impl Sub<Int4> for i32 {
    type Output = Int4;
    fn sub(self, rhs: Int4) -> Int4 {
        Int4 { 
            x: self - rhs.x, 
            y: self - rhs.y, 
            z: self - rhs.z, 
            w: self - rhs.w 
        }
    }
}

impl SubAssign for Int4 {
    fn sub_assign(&mut self, rhs: Int4) {
        self.x -= rhs.x; 
        self.y -= rhs.y; 
        self.z -= rhs.z; 
        self.w -= rhs.w;
    }
}
impl SubAssign<i32> for Int4 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; 
        self.y -= rhs; 
        self.z -= rhs; 
        self.w -= rhs;
    }
}

// ======= MUL ======= 
impl Mul<i32> for Int4 {
    type Output = Int4;
    fn mul(self, rhs: i32) -> Int4 {
        Int4 { 
            x: self.x * rhs, 
            y: self.y * rhs, 
            z: self.z * rhs, 
            w: self.w * rhs 
        }
    }
}
impl Mul<Int4> for i32 {
    type Output = Int4;
    fn mul(self, rhs: Int4) -> Int4 {
        Int4 { 
            x: self * rhs.x, 
            y: self * rhs.y, 
            z: self * rhs.z, 
            w: self * rhs.w 
        }
    }
}

impl MulAssign<i32> for Int4 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; 
        self.y *= rhs; 
        self.z *= rhs; 
        self.w *= rhs;
    }
}

// ======= DIV ======= 
impl Div<i32> for Int4 {
    type Output = Int4;
    fn div(self, rhs: i32) -> Int4 {
        Int4 { 
            x: self.x / rhs, 
            y: self.y / rhs, 
            z: self.z / rhs, 
            w: self.w / rhs 
        }
    }
}

impl DivAssign<i32> for Int4 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; 
        self.y /= rhs; 
        self.z /= rhs; 
        self.w /= rhs;
    }
}