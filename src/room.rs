use crate::{
    common::{Rgb, Vec2},
    view::primitives::{Line, Primitive},
};
pub type RoomId = usize;

#[derive(Clone)]
pub struct Room {
    pub id: Option<RoomId>,
    verts: Vec<Vec2<i32>>,
    pub name: String,
    pub notes: String,
    wall_width: f64,
}

impl Room {
    pub fn new(id: Option<RoomId>) -> Self {
        Self {
            id: id,
            verts: vec![],
            name: "New Room".to_owned(),
            notes: String::new(),
            wall_width: 10.0,
        }
    }

    pub fn draw(&self, next_vert: Option<Vec2<i32>>) -> Vec<Box<dyn Primitive>> {
        let mut verts = self.verts.clone();
        match next_vert {
            Some(v) => verts.push(v),
            None => (),
        }
        let mut lines = Vec::<Box<dyn Primitive>>::new();
        for i in 1..verts.len() {
            let line: Box<dyn Primitive> = Box::new(Line {
                from: Vec2 {
                    x: verts[i - 1].x as f64,
                    y: verts[i - 1].y as f64,
                },
                to: Vec2 {
                    x: verts[i].x as f64,
                    y: verts[i].y as f64,
                },
                color: Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                },
                width: self.wall_width,
            });
            lines.push(line);
        }
        if verts.len() > 1 {
            let line: Box<dyn Primitive> = Box::new(Line {
                from: Vec2 {
                    x: verts[verts.len() - 1].x as f64,
                    y: verts[verts.len() - 1].y as f64,
                },
                to: Vec2 {
                    x: verts[0].x as f64,
                    y: verts[0].y as f64,
                },
                color: Rgb {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                },
                width: self.wall_width,
            });
            lines.push(line);
        }
        lines
    }

    pub fn append(&mut self, vert: Vec2<i32>) {
        self.verts.push(vert)
    }
}
