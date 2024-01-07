use crate::{
    chamber::ChamberId,
    common::{Rgb, Vec2},
    config::{GRID_SIZE, WALL_WIDTH},
    view::primitives::{Circle, Line, Polygon, Primitive},
};

pub type ObjectId = u32;

#[derive(Clone, Copy)]
pub enum ObjectStyle {
    Blocker,
    Stairs,
    Round,
}

impl ObjectStyle {
    pub fn to_str(&self) -> String {
        match self {
            ObjectStyle::Blocker => "Blocker".to_owned(),
            ObjectStyle::Stairs => "Stairs".to_owned(),
            ObjectStyle::Round => "Round".to_owned(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "Blocker" => ObjectStyle::Blocker,
            "Stairs" => ObjectStyle::Stairs,
            "Round" => ObjectStyle::Round,
            _ => todo!(),
        }
    }
}

pub struct ObjectDrawOptions {
    pub color: Option<Rgb>,
}

impl ObjectDrawOptions {
    pub fn empty() -> ObjectDrawOptions {
        ObjectDrawOptions { color: None }
    }
}

#[derive(Clone)]
pub struct Object {
    pub id: ObjectId,
    pub name: String,
    pub notes: String,
    pub hidden: bool,
    pub style: ObjectStyle,
    pub pos: Vec2<i32>,
    pub part_of: Option<ChamberId>,
}

impl Object {
    pub fn new(pos: Vec2<i32>, part_of: Option<ObjectId>) -> Self {
        Self {
            id: 1,
            name: "".to_owned(),
            notes: "".to_owned(),
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

        match self.style {
            ObjectStyle::Blocker => self.draw_blocker(color),
            ObjectStyle::Stairs => self.draw_stairs(color),
            ObjectStyle::Round => self.draw_round(color),
        }
    }

    pub fn contains(&self, pos: Vec2<f64>) -> bool {
        let s: f64 = GRID_SIZE as f64;
        let obj_pos: Vec2<f64> = self.pos.into();
        pos.x >= obj_pos.x
            && pos.y >= obj_pos.y
            && pos.x <= (obj_pos.x + s)
            && pos.y <= (obj_pos.y + s)
    }

    fn draw_blocker(&self, color: Rgb) -> Vec<Box<dyn Primitive>> {
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
                dashed: self.hidden,
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
                dashed: self.hidden,
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

    fn draw_stairs(&self, color: Rgb) -> Vec<Box<dyn Primitive>> {
        let s: f64 = GRID_SIZE as f64;
        let c: f64 = GRID_SIZE as f64 / 2.0;

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
            Box::new(Line {
                color: color,
                dashed: false,
                from: Vec2 {
                    x: self.pos.x as f64 + c - 0.8 * c,
                    y: self.pos.y as f64 + 0.2 * s,
                },
                to: Vec2 {
                    x: self.pos.x as f64 + c + 0.8 * c,
                    y: self.pos.y as f64 + 0.2 * s,
                },
                width: WALL_WIDTH,
            }),
            Box::new(Line {
                color: color,
                dashed: false,
                from: Vec2 {
                    x: self.pos.x as f64 + c - 0.6 * c,
                    y: self.pos.y as f64 + 0.4 * s,
                },
                to: Vec2 {
                    x: self.pos.x as f64 + c + 0.6 * c,
                    y: self.pos.y as f64 + 0.4 * s,
                },
                width: WALL_WIDTH,
            }),
            Box::new(Line {
                color: color,
                dashed: false,
                from: Vec2 {
                    x: self.pos.x as f64 + c - 0.4 * c,
                    y: self.pos.y as f64 + 0.6 * s,
                },
                to: Vec2 {
                    x: self.pos.x as f64 + c + 0.4 * c,
                    y: self.pos.y as f64 + 0.6 * s,
                },
                width: WALL_WIDTH,
            }),
            Box::new(Line {
                color: color,
                dashed: false,
                from: Vec2 {
                    x: self.pos.x as f64 + c - 0.2 * c,
                    y: self.pos.y as f64 + 0.8 * s,
                },
                to: Vec2 {
                    x: self.pos.x as f64 + c + 0.2 * c,
                    y: self.pos.y as f64 + 0.8 * s,
                },
                width: WALL_WIDTH,
            }),
        ]
    }

    fn draw_round(&self, color: Rgb) -> Vec<Box<dyn Primitive>> {
        let pos: Vec2<f64> = self.pos.into();
        vec![
            Box::new(Circle {
                at: pos
                    + Vec2::<f64> {
                        x: GRID_SIZE as f64 / 2.0,
                        y: GRID_SIZE as f64 / 2.0,
                    },
                radius: GRID_SIZE as f64 / 2.0,
                width: WALL_WIDTH,
                color: color,
                dashed: self.hidden,
            }),
            Box::new(Circle {
                at: pos
                    + Vec2::<f64> {
                        x: GRID_SIZE as f64 / 2.0,
                        y: GRID_SIZE as f64 / 2.0,
                    },
                radius: GRID_SIZE as f64 / 4.0,
                width: WALL_WIDTH,
                color: color,
                dashed: false,
            }),
        ]
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
