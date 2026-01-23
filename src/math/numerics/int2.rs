use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

impl Int2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// ======= ADD ======= 
impl Add for Int2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Add<i32> for Int2 {
    type Output = Self;
    fn add(self, rhs: i32) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}
impl Add<Int2> for i32 {
    type Output = Int2;
    fn add(self, rhs: Int2) -> Int2 {
        Int2 { x: self + rhs.x, y: self + rhs.y }
    }
}

impl AddAssign for Int2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x; self.y += rhs.y;
    }
}
impl AddAssign<i32> for Int2 {
    fn add_assign(&mut self, rhs: i32) {
        self.x += rhs; self.y += rhs;
    }
}

// ======= SUB ======= 
impl Sub for Int2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Sub<i32> for Int2 {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}
impl Sub<Int2> for i32 {
    type Output = Int2;
    fn sub(self, rhs: Int2) -> Int2 {
        Int2 { x: self - rhs.x, y: self - rhs.y }
    }
}

impl SubAssign for Int2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x; self.y -= rhs.y;
    }
}
impl SubAssign<i32> for Int2 {
    fn sub_assign(&mut self, rhs: i32) {
        self.x -= rhs; self.y -= rhs;
    }
}

// ======= MUL =======
impl Mul<i32> for Int2 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}
impl Mul<Int2> for i32 {
    type Output = Int2;
    fn mul(self, rhs: Int2) -> Int2 {
        Int2 { x: self * rhs.x, y: self * rhs.y }
    }
}

impl MulAssign<i32> for Int2 {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs; self.y *= rhs;
    }
}

// ======= DIV =======
impl Div<i32> for Int2 {
    type Output = Self;
    fn div(self, rhs: i32) -> Self {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<i32> for Int2 {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs; self.y /= rhs;
    }
}