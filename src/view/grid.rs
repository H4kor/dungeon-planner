use crate::common::{Rgb, Vec2};
use crate::view::primitives::{Line, Primitive};

#[derive(Clone, Copy)]
pub struct Grid {
    size: Vec2<i32>,
    color: Rgb,
    width: f64,
}

impl Grid {
    pub fn new() -> Self {
        Grid {
            size: Vec2 { x: 50, y: 50 },
            color: Rgb {
                r: 0.0,
                b: 0.0,
                g: 0.0,
            },
            width: 1.0,
        }
    }

    pub fn draw(self, min: Vec2<i32>, max: Vec2<i32>) -> Vec<Box<dyn Primitive>> {
        let start_index = min / self.size + Vec2 { x: -1, y: -1 };
        let end_index = (max / self.size) + Vec2 { x: 1, y: 1 };
        let mut lines = Vec::<Box<dyn Primitive>>::new();
        for x_idx in start_index.x..end_index.x {
            let line: Box<dyn Primitive> = Box::new(Line {
                from: Vec2 {
                    x: (x_idx * self.size.x) as f64,
                    y: min.y as f64,
                },
                to: Vec2 {
                    x: (x_idx * self.size.x) as f64,
                    y: max.y as f64,
                },
                color: self.color,
                width: self.width,
            });
            lines.push(line);
        }
        for y_idx in start_index.y..end_index.y {
            let line: Box<dyn Primitive> = Box::new(Line {
                from: Vec2 {
                    x: min.x as f64,
                    y: (y_idx * self.size.x) as f64,
                },
                to: Vec2 {
                    x: max.x as f64,
                    y: (y_idx * self.size.x) as f64,
                },
                color: self.color,
                width: self.width,
            });
            lines.push(line);
        }

        lines
    }
}
