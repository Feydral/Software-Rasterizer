use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

impl Float2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 { self } else { self / len }
    }
}

// ======= ADD ======= 
impl Add for Float2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<f32> for Float2 {
    type Output = Self;
    fn add(self, rhs: f32) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<Float2> for f32 {
    type Output = Float2;
    fn add(self, rhs: Float2) -> Float2 {
        Float2 { x: self + rhs.x, y: self + rhs.y }
    }
}

impl AddAssign for Float2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<f32> for Float2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs; self.y += rhs;
    }
}

// ======= SUB ======= 
impl Sub for Float2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<f32> for Float2 {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<Float2> for f32 {
    type Output = Float2;
    fn sub(self, rhs: Float2) -> Float2 {
        Float2 { x: self - rhs.x, y: self - rhs.y }
    }
}

impl SubAssign for Float2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<f32> for Float2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs; self.y -= rhs;
    }
}

// ======= MUL =======
impl Mul<f32> for Float2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<Float2> for f32 {
    type Output = Float2;
    fn mul(self, rhs: Float2) -> Float2 {
        Float2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl MulAssign<f32> for Float2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs; self.y *= rhs;
    }
}

// ======= DIV =======
impl Div<f32> for Float2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<f32> for Float2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs; self.y /= rhs;
    }
}