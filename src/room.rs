use crate::{
    common::{Rgb, Vec2},
    view::primitives::{Polygon, Primitive},
};
pub type RoomId = u32;
pub type WallId = u32;

/// One wall of a room
/// Has an id and two points defining its shape
/// The id is only unique within a room
/// Not meant to be stored, as these are derived from a Room
#[derive(Clone, Copy)]
pub struct Wall {
    pub id: RoomId,
    pub p1: Vec2<i32>,
    pub p2: Vec2<i32>,
}

/// A Room is part of a Dungeon
/// It has a shape and further information, such as name and notes
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
            points: verts.iter().map(|p| Into::<Vec2<f64>>::into(*p)).collect(),
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

    pub fn walls(&self) -> Vec<Wall> {
        let mut walls = vec![];

        if self.verts.len() > 1 {
            let mut prev = self.verts[0];
            for (i, v) in self.verts[1..].iter().enumerate() {
                walls.push(Wall {
                    id: i as WallId,
                    p1: prev,
                    p2: *v,
                });
                prev = *v;
            }
            walls.push(Wall {
                id: walls.len() as RoomId,
                p1: prev,
                p2: self.verts[0],
            })
        }
        walls
    }
}

impl Wall {
    pub fn distance(&self, p: Vec2<f64>) -> f64 {
        // Return minimum distance between line segment vw and point p
        // https://stackoverflow.com/a/1501725/1224467
        let v: Vec2<f64> = self.p1.into();
        let w: Vec2<f64> = self.p2.into();
        let l2 = (w - v).sqr_len(); // i.e. |w-v|^2 -  avoid a sqrt
        if l2 == 0.0 {
            return (p - v).len(); // v == w case
        }
        // Consider the line extending the segment, parameterized as v + t (w - v).
        // We find projection of point p onto the line.
        // It falls where t = [(p-v) . (w-v)] / |w-v|^2
        // We clamp t from [0,1] to handle points outside the segment vw.
        let t = f64::max(0.0, f64::min(1.0, (p - v).dot(w - v) / l2));
        let projection = v + (w - v) * Vec2 { x: t, y: t }; // Projection falls on the segment
        (p - projection).len()
    }
}

#[cfg(test)]
mod tests {
    use crate::common::Vec2;

    use super::{Room, Wall};

    #[test]
    fn test_walls_now_verts() {
        let r = Room::new(None);
        let walls = r.walls();
        assert_eq!(walls.len(), 0);
    }

    #[test]
    fn test_walls_one_verts() {
        let mut r = Room::new(None);
        r.verts = vec![Vec2 { x: 1, y: 1 }];
        let walls = r.walls();
        assert_eq!(walls.len(), 0);
    }

    #[test]
    fn test_walls_two_verts() {
        let mut r = Room::new(None);
        r.verts = vec![Vec2 { x: 1, y: 1 }, Vec2 { x: 2, y: 2 }];
        let walls = r.walls();
        assert_eq!(walls.len(), 2);

        assert_eq!(walls[0].id, 0);
        assert_eq!(walls[0].p1, Vec2 { x: 1, y: 1 });
        assert_eq!(walls[0].p2, Vec2 { x: 2, y: 2 });

        assert_eq!(walls[1].id, 1);
        assert_eq!(walls[1].p1, Vec2 { x: 2, y: 2 });
        assert_eq!(walls[1].p2, Vec2 { x: 1, y: 1 });
    }

    #[test]
    fn test_walls_three_verts() {
        let mut r = Room::new(None);
        r.verts = vec![
            Vec2 { x: 1, y: 1 },
            Vec2 { x: 2, y: 2 },
            Vec2 { x: 3, y: 3 },
        ];
        let walls = r.walls();
        assert_eq!(walls.len(), 3);
        assert_eq!(walls[0].id, 0);
        assert_eq!(walls[0].p1, Vec2 { x: 1, y: 1 });
        assert_eq!(walls[0].p2, Vec2 { x: 2, y: 2 });

        assert_eq!(walls[1].id, 1);
        assert_eq!(walls[1].p1, Vec2 { x: 2, y: 2 });
        assert_eq!(walls[1].p2, Vec2 { x: 3, y: 3 });

        assert_eq!(walls[2].id, 2);
        assert_eq!(walls[2].p1, Vec2 { x: 3, y: 3 });
        assert_eq!(walls[2].p2, Vec2 { x: 1, y: 1 });
    }

    #[test]
    fn wall_dist() {
        let w = Wall {
            id: 0,
            p1: Vec2 { x: 0, y: 0 },
            p2: Vec2 { x: 1, y: 0 },
        };

        assert_eq!(w.distance(Vec2 { x: 1.0, y: 0.0 }), 0.0);
        assert_eq!(w.distance(Vec2 { x: 0.0, y: 0.0 }), 0.0);
        assert_eq!(w.distance(Vec2 { x: 1.0, y: 1.0 }), 1.0);
        assert_eq!(w.distance(Vec2 { x: 2.0, y: 0.0 }), 1.0);
        assert_eq!(w.distance(Vec2 { x: -2.0, y: 0.0 }), 2.0);
    }
}
