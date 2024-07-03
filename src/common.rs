use std::ops::{Add, AddAssign, BitAnd, BitAndAssign, Div, Mul, Sub};

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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line {
    pub a: Vec2<f64>,
    pub b: Vec2<f64>,
}

#[derive(Debug)]
pub struct BBox {
    pub min: Vec2<f64>,
    pub max: Vec2<f64>,
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

/**
 * BBox
 */

impl BitAnd<BBox> for BBox {
    type Output = BBox;

    fn bitand(self, rhs: BBox) -> Self::Output {
        BBox {
            min: Vec2 {
                x: f64::min(self.min.x, rhs.min.x),
                y: f64::min(self.min.y, rhs.min.y),
            },
            max: Vec2 {
                x: f64::max(self.max.x, rhs.max.x),
                y: f64::max(self.max.y, rhs.max.y),
            },
        }
    }
}

impl BitAndAssign<BBox> for BBox {
    fn bitand_assign(&mut self, rhs: BBox) {
        self.min = Vec2 {
            x: f64::min(self.min.x, rhs.min.x),
            y: f64::min(self.min.y, rhs.min.y),
        };
        self.max = Vec2 {
            x: f64::max(self.max.x, rhs.max.x),
            y: f64::max(self.max.y, rhs.max.y),
        };
    }
}

impl BBox {
    pub fn is_valid(&self) -> bool {
        self.min.x.is_finite()
            && self.min.y.is_finite()
            && self.max.x.is_finite()
            && self.max.y.is_finite()
    }
}

#[cfg(test)]
mod tests {
    use super::{BBox, Vec2};

    #[test]
    fn test_bbox_and() {
        let a = BBox {
            min: Vec2 { x: 0.0, y: 1.0 },
            max: Vec2 { x: 2.0, y: 3.0 },
        };
        let b = BBox {
            min: Vec2 { x: 4.0, y: 5.0 },
            max: Vec2 { x: 6.0, y: 7.0 },
        };

        let c = a & b;
        assert_eq!(c.min.x, 0.0);
        assert_eq!(c.min.y, 1.0);
        assert_eq!(c.max.x, 6.0);
        assert_eq!(c.max.y, 7.0);
    }

    #[test]
    fn test_bbox_and_inplace() {
        let mut a = BBox {
            min: Vec2 { x: 0.0, y: 1.0 },
            max: Vec2 { x: 2.0, y: 3.0 },
        };
        let b = BBox {
            min: Vec2 { x: 4.0, y: 5.0 },
            max: Vec2 { x: 6.0, y: 7.0 },
        };

        a &= b;
        assert_eq!(a.min.x, 0.0);
        assert_eq!(a.min.y, 1.0);
        assert_eq!(a.max.x, 6.0);
        assert_eq!(a.max.y, 7.0);
    }

    #[test]
    fn test_invalid_bbox() {
        let a = BBox {
            min: Vec2 {
                x: 1.0 / 0.0,
                y: 1.0 / 0.0,
            },
            max: Vec2 {
                x: 1.0 / 0.0,
                y: 1.0 / 0.0,
            },
        };
        assert_eq!(a.is_valid(), false)
    }

    #[test]
    fn test_valid_bbox() {
        let a = BBox {
            min: Vec2 { x: 1.0, y: 1.0 },
            max: Vec2 { x: 1.0, y: 1.0 },
        };
        assert_eq!(a.is_valid(), true)
    }
}
