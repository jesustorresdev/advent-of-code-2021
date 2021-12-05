use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;

#[derive(Copy, Clone)]
pub struct Vector2 {
    x: i16,
    y: i16
}

impl Vector2 {
    pub fn new(x: i16, y: i16) -> Self {
        Self {
            x: x,
            y: y
        } 
    }

    pub fn zero() -> Self {
        Self {
            x: 0,
            y: 0
        }
    }

    pub fn multiply(self) -> i32 {
        self.x as i32 * self.y as i32
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self::zero()
    }
}

impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vector2::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vector2::new(self.x + rhs.x, self.y + rhs.y);
    }
}

impl Mul<i16> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: i16) -> Self {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}
