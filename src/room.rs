use crate::{
    common::{Rgb, Vec2},
    view::primitives::{Polygon, Primitive},
};
pub type RoomId = u32;

#[derive(Clone)]
pub struct Room {
    pub id: Option<RoomId>,
    verts: Vec<Vec2<i32>>,
    pub name: String,
    pub notes: String,
    wall_color: Rgb,
    room_color: Rgb,
    wall_width: f64,
}

impl Room {
    pub fn new(id: Option<RoomId>) -> Self {
        Self {
            id: id,
            verts: vec![],
            name: "New Room".to_owned(),
            notes: String::new(),
            wall_color: Rgb {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            room_color: Rgb {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            wall_width: 7.0,
        }
    }

    pub fn draw(&self, next_vert: Option<Vec2<i32>>) -> Vec<Box<dyn Primitive>> {
        let mut verts = self.verts.clone();
        match next_vert {
            Some(v) => verts.push(v),
            None => (),
        }
        let mut lines = Vec::<Box<dyn Primitive>>::new();

        lines.push(Box::new(Polygon {
            points: verts
                .iter()
                .map(|p| Vec2::<f64> {
                    x: p.x as f64,
                    y: p.y as f64,
                })
                .collect(),
            fill_color: self.room_color,
            fill_opacity: 0.3,
            stroke_color: self.wall_color,
            stroke_width: self.wall_width,
        }));

        lines
    }

    pub fn append(&mut self, vert: Vec2<i32>) {
        self.verts.push(vert)
    }
}
