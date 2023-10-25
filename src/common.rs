use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy)]
pub struct Rgb {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl<T: Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
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
