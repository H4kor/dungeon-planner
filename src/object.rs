use crate::{
    chamber::ChamberId,
    common::{Rgb, Vec2},
    config::{GRID_SIZE, WALL_WIDTH},
    view::primitives::{Line, Polygon, Primitive},
};

pub type ObjectId = u32;

#[derive(Clone, Copy)]
pub enum ObjectStyle {
    Blocker,
    Stairs,
    Round,
}

pub struct ObjectDrawOptions {
    pub color: Option<Rgb>,
}

impl ObjectDrawOptions {
    pub(crate) fn empty() -> ObjectDrawOptions {
        ObjectDrawOptions { color: None }
    }
}

#[derive(Clone, Copy)]
pub struct Object {
    pub id: ObjectId,
    pub style: ObjectStyle,
    pub pos: Vec2<i32>,
    pub hidden: bool,
    pub part_of: Option<ChamberId>,
}

impl Object {
    pub fn new(pos: Vec2<i32>, part_of: Option<ObjectId>) -> Self {
        Self {
            id: 1,
            style: ObjectStyle::Blocker,
            pos: pos,
            hidden: false,
            part_of: part_of,
        }
    }

    pub fn draw(&self, options: ObjectDrawOptions) -> Vec<Box<dyn Primitive>> {
        let color = options.color.unwrap_or(Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        });

        vec![
            // draw box
            Box::new(Polygon {
                dashed: self.hidden,
                fill_color: color,
                fill_opacity: 0.0,
                stroke_color: color,
                stroke_width: WALL_WIDTH,
                points: vec![
                    Vec2 {
                        x: self.pos.x as f64,
                        y: self.pos.y as f64,
                    },
                    Vec2 {
                        x: (self.pos.x + GRID_SIZE) as f64,
                        y: self.pos.y as f64,
                    },
                    Vec2 {
                        x: (self.pos.x + GRID_SIZE) as f64,
                        y: (self.pos.y + GRID_SIZE) as f64,
                    },
                    Vec2 {
                        x: self.pos.x as f64,
                        y: (self.pos.y + GRID_SIZE) as f64,
                    },
                ],
            }),
            // draw x in box
            Box::new(Line {
                color: color,
                from: Vec2 {
                    x: self.pos.x as f64,
                    y: self.pos.y as f64,
                },
                to: Vec2 {
                    x: (self.pos.x + GRID_SIZE) as f64,
                    y: (self.pos.y + GRID_SIZE) as f64,
                },
                width: WALL_WIDTH,
            }),
            Box::new(Line {
                color: color,
                from: Vec2 {
                    x: (self.pos.x + GRID_SIZE) as f64,
                    y: self.pos.y as f64,
                },
                to: Vec2 {
                    x: self.pos.x as f64,
                    y: (self.pos.y + GRID_SIZE) as f64,
                },
                width: WALL_WIDTH,
            }),
        ]
    }

    pub(crate) fn contains(&self, pos: Vec2<f64>) -> bool {
        let s: f64 = GRID_SIZE as f64;
        let obj_pos: Vec2<f64> = self.pos.into();
        pos.x >= obj_pos.x
            && pos.y >= obj_pos.y
            && pos.x <= (obj_pos.x + s)
            && pos.y <= (obj_pos.y + s)
    }
}

#[cfg(test)]
mod test {
    use crate::common::Vec2;

    use super::Object;

    #[test]
    fn test_contains() {
        let obj = Object::new(Vec2 { x: 0, y: 0 }, None);

        assert_eq!(obj.contains(Vec2 { x: 0.0, y: 0.0 }), true);
        assert_eq!(obj.contains(Vec2 { x: 20.0, y: 0.0 }), true);
        assert_eq!(obj.contains(Vec2 { x: 0.0, y: 20.0 }), true);
        assert_eq!(obj.contains(Vec2 { x: 20.0, y: 20.0 }), true);
        assert_eq!(obj.contains(Vec2 { x: 50.0, y: 50.0 }), true);
        assert_eq!(obj.contains(Vec2 { x: -20.0, y: 20.0 }), false);
        assert_eq!(obj.contains(Vec2 { x: 20.0, y: -20.0 }), false);
        assert_eq!(obj.contains(Vec2 { x: -20.0, y: -20.0 }), false);
        assert_eq!(obj.contains(Vec2 { x: 51.0, y: 51.0 }), false);
    }
}
