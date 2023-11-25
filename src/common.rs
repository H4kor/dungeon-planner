use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub struct Line {
    pub a: Vec2<f64>,
    pub b: Vec2<f64>,
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Into<f64> + Copy> Vec2<T> {
    pub fn sqr_len(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y)).into()
    }
    pub fn len(&self) -> f64 {
        f64::sqrt(self.sqr_len())
    }
    pub fn dot(&self, other: Vec2<T>) -> T {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        Vec2 { x, y }
    }
}

impl<T: Sub<Output = T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Vec2 { x, y }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl<T: Div<Output = T>> Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, rhs: Vec2<T>) -> Vec2<T> {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        Vec2 { x, y }
    }
}

impl<T: Mul<Output = T>> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        Vec2 { x, y }
    }
}

impl Mul<Vec2<f64>> for f64 {
    type Output = Vec2<f64>;

    fn mul(self, rhs: Vec2<f64>) -> Self::Output {
        Vec2 {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl Into<Vec2<f64>> for Vec2<i32> {
    fn into(self) -> Vec2<f64> {
        Vec2 {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl Into<Vec2<i32>> for Vec2<f64> {
    fn into(self) -> Vec2<i32> {
        Vec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

/**
 * LINE
 */

impl Line {
    pub fn min(&self) -> Vec2<f64> {
        return Vec2 {
            x: f64::min(self.a.x, self.b.x),
            y: f64::min(self.a.y, self.b.y),
        };
    }

    pub fn max(&self) -> Vec2<f64> {
        return Vec2 {
            x: f64::max(self.a.x, self.b.x),
            y: f64::max(self.a.y, self.b.y),
        };
    }
}
