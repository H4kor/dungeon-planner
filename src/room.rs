use crate::{
    common::{Line, Rgb, Vec2},
    config::{ACTIVE_ROOM_COLOR, DEFAULT_ROOM_COLOR, WALL_WIDTH},
    view::primitives::{Polygon, Primitive, Text},
};
pub type RoomId = u32;
pub type WallId = u32;

/// One wall of a room
/// Has an id and two points defining its shape
/// The id is only unique within a room. RoomIds are persistent and not ordered
/// RoomIds are reused
/// Not meant to be stored, as these are derived from a Room
#[derive(Clone, Copy, Debug)]
pub struct Wall {
    pub id: RoomId,
    pub p1: Vec2<i32>,
    pub p2: Vec2<i32>,
}

pub struct NextVert {
    pub in_wall_id: Option<WallId>,
    pub pos: Vec2<i32>,
}

pub struct RoomDrawOptions {
    pub color: Option<Rgb>,
    pub fill: Option<bool>,
}

/// A Room is part of a Dungeon
/// It has a shape and further information, such as name and notes
#[derive(Clone)]
pub struct Room {
    pub id: Option<RoomId>,
    walls: Vec<Wall>,
    first_vert: Option<Vec2<i32>>,
    pub name: String,
    pub notes: String,
    room_color: Rgb,
}

impl Room {
    pub fn new(id: Option<RoomId>) -> Self {
        Self {
            id: id,
            walls: vec![],
            first_vert: None,
            name: "New Room".to_owned(),
            notes: String::new(),
            room_color: DEFAULT_ROOM_COLOR,
        }
    }

    pub fn draw(
        &self,
        next_vert: Option<NextVert>,
        active: bool,
        options: Option<RoomDrawOptions>,
    ) -> Vec<Box<dyn Primitive>> {
        let mut walls = self.walls.clone();
        let mut show_room_number = true;
        match next_vert {
            Some(v) => match v.in_wall_id {
                Some(wall_id) => {
                    let idx = walls.iter().position(|w| w.id == wall_id).unwrap().clone();
                    let wall = walls[idx];
                    let (w1, w2) = wall.split(v.pos);
                    walls[idx] = w1;
                    walls.insert(idx + 1, w2);
                    show_room_number = false;
                }
                None => {
                    if walls.len() > 0 {
                        let idx = walls.len() - 1;
                        let wall = walls[idx];
                        let (w1, w2) = wall.split(v.pos);
                        walls[idx] = w1;
                        walls.insert(idx + 1, w2);
                        show_room_number = false;
                    }
                }
            },
            None => (),
        }

        let color = match active {
            false => match options {
                Some(RoomDrawOptions {
                    color: Some(c),
                    fill: _,
                }) => c,
                _ => self.room_color,
            },
            true => ACTIVE_ROOM_COLOR,
        };

        let mut prims = Vec::<Box<dyn Primitive>>::new();
        let poly = Box::new(Polygon {
            points: walls
                .iter()
                .map(|p| Into::<Vec2<f64>>::into(p.p1))
                .collect(),
            fill_color: color,
            fill_opacity: match options {
                Some(RoomDrawOptions {
                    color: _,
                    fill: Some(false),
                }) => 0.0,
                _ => 0.3,
            },
            stroke_color: color,
            stroke_width: WALL_WIDTH,
        });
        let bbox = poly.bbox();
        prims.push(poly);

        if show_room_number {
            // get bbox of polygon
            // iterate over each cell in BBox
            // if in polygon
            // calc min distance to polygon
            // take cell with max min dist which is in polygon
            let mut max_min_dist = f64::NEG_INFINITY;
            let mut best_p = None;

            let grid = 50.0;
            let x_steps: u32 = ((bbox.max.x - bbox.min.x) / grid).ceil() as u32;
            let y_steps: u32 = ((bbox.max.y - bbox.min.y) / grid).ceil() as u32;
            for x_i in 0..x_steps {
                for y_i in 0..y_steps {
                    let p = bbox.min
                        + Vec2::<f64> {
                            x: (x_i as f64) * grid + 25.0,
                            y: (y_i as f64) * grid + 25.0,
                        };
                    if self.contains_point(p) {
                        let d = self
                            .walls()
                            .iter()
                            .map(|w| w.distance(p))
                            .reduce(f64::min)
                            .unwrap_or(f64::NEG_INFINITY);
                        if d > max_min_dist {
                            max_min_dist = d;
                            best_p = Some(p);
                        }
                    }
                }
            }

            if let Some(p) = best_p {
                prims.push(Box::new(Text {
                    at: Into::<Vec2<f64>>::into(p),
                    text: self.id.unwrap_or(0).to_string(),
                    color: color,
                    size: 25.0,
                }));
            }
        }

        prims
    }

    fn next_wall_id(&self) -> WallId {
        self.walls.iter().map(|w| w.id).max().unwrap_or(0) + 1
    }

    pub fn append(&mut self, vert: Vec2<i32>) {
        // split last wall
        if self.walls.len() == 0 && self.first_vert == None {
            self.first_vert = Some(vert);
        } else if self.walls.len() == 0 {
            self.walls.push(Wall {
                id: self.next_wall_id(),
                p1: self.first_vert.unwrap(),
                p2: vert,
            });
            self.walls.push(Wall {
                id: self.next_wall_id(),
                p1: vert,
                p2: self.first_vert.unwrap(),
            })
        } else {
            let (w1, mut w2) = self.walls.pop().unwrap().split(vert);
            self.walls.push(w1);
            w2.id = self.next_wall_id();
            self.walls.push(w2);
        }
    }

    pub fn walls(&self) -> &Vec<Wall> {
        &self.walls
    }

    fn lines(&self) -> Vec<Line> {
        self.walls
            .iter()
            .map(|w| Line {
                a: w.p1.into(),
                b: w.p2.into(),
            })
            .collect()
    }

    pub fn contains_point(&self, pos: Vec2<f64>) -> bool {
        let mut crossings: i32 = 0;
        for line in self.lines() {
            // filter out lines not containing pos.y
            if line.min().y > pos.y {
                continue;
            }
            if line.max().y < pos.y {
                continue;
            }
            // calculate x of line at pos.y
            let d = line.b - line.a;
            let f = (pos.y - line.a.y) / d.y;
            let c = line.a + f * d;
            if c.x < pos.x {
                // line lines "before" pos
                crossings += 1;
            }
        }
        // uneven crossing == inside
        crossings % 2 == 1
    }

    pub(crate) fn nearest_wall(&self, pos: Vec2<f64>) -> Option<Wall> {
        let mut min_wall = None;
        let mut min_d = f64::INFINITY;
        for wall in self.walls().iter() {
            let d = wall.distance(pos);
            if d < min_d {
                min_wall = Some(*wall);
                min_d = d;
            }
        }
        min_wall
    }

    pub(crate) fn split(&mut self, wall_id: u32, pos: Vec2<i32>) {
        let idx = self
            .walls
            .iter()
            .position(|w| w.id == wall_id)
            .unwrap()
            .clone();

        let (w1, mut w2) = self.walls[idx].split(pos);
        w2.id = self.next_wall_id();

        self.walls[idx] = w1;
        self.walls.insert(idx + 1, w2);
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

    fn split(&self, vert: Vec2<i32>) -> (Wall, Wall) {
        (
            Wall {
                id: self.id,
                p1: self.p1,
                p2: vert,
            },
            Wall {
                id: self.id,
                p1: vert,
                p2: self.p2,
            },
        )
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
        r.append(Vec2 { x: 1, y: 1 });
        let walls = r.walls();
        assert_eq!(walls.len(), 0);
    }

    #[test]
    fn test_walls_two_verts() {
        let mut r = Room::new(None);
        r.append(Vec2 { x: 1, y: 1 });
        r.append(Vec2 { x: 2, y: 2 });
        let walls = r.walls();
        assert_eq!(walls.len(), 2);

        assert_eq!(walls[0].p1, Vec2 { x: 1, y: 1 });
        assert_eq!(walls[0].p2, Vec2 { x: 2, y: 2 });

        assert_eq!(walls[1].p1, Vec2 { x: 2, y: 2 });
        assert_eq!(walls[1].p2, Vec2 { x: 1, y: 1 });
    }

    #[test]
    fn test_walls_three_verts() {
        let mut r = Room::new(None);
        r.append(Vec2 { x: 1, y: 1 });
        r.append(Vec2 { x: 2, y: 2 });
        r.append(Vec2 { x: 3, y: 3 });
        let walls = r.walls();
        assert_eq!(walls.len(), 3);
        assert_eq!(walls[0].p1, Vec2 { x: 1, y: 1 });
        assert_eq!(walls[0].p2, Vec2 { x: 2, y: 2 });

        assert_eq!(walls[1].p1, Vec2 { x: 2, y: 2 });
        assert_eq!(walls[1].p2, Vec2 { x: 3, y: 3 });

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

    #[test]
    fn contains_1() {
        let mut r = Room::new(None);
        r.append(Vec2 { x: 0, y: 0 });
        r.append(Vec2 { x: 0, y: 10 });
        r.append(Vec2 { x: 10, y: 10 });
        r.append(Vec2 { x: 10, y: 0 });

        assert_eq!(r.contains_point(Vec2 { x: 5.0, y: 5.0 }), true);
        assert_eq!(r.contains_point(Vec2 { x: -5.0, y: 5.0 }), false);
        assert_eq!(r.contains_point(Vec2 { x: 5.0, y: -5.0 }), false);
    }

    #[test]
    fn contains_2() {
        let mut r = Room::new(None);
        // U shape 150 - 250   350 - 450
        // Y 350 - 650
        r.append(Vec2 { x: 150, y: 350 });
        r.append(Vec2 { x: 250, y: 350 });
        r.append(Vec2 { x: 250, y: 550 });
        r.append(Vec2 { x: 350, y: 550 });
        r.append(Vec2 { x: 350, y: 350 });
        r.append(Vec2 { x: 450, y: 350 });
        r.append(Vec2 { x: 450, y: 650 });
        r.append(Vec2 { x: 150, y: 650 });

        assert_eq!(r.contains_point(Vec2 { x: 200., y: 400.0 }), true);
        assert_eq!(r.contains_point(Vec2 { x: 400., y: 400.0 }), true);
        assert_eq!(r.contains_point(Vec2 { x: 300., y: 400.0 }), false);
        assert_eq!(r.contains_point(Vec2 { x: 300., y: 600.0 }), true);
    }

    #[test]
    fn contains_3() {
        let mut r = Room::new(None);
        // triangle
        r.append(Vec2 { x: 100, y: 0 });
        r.append(Vec2 { x: 50, y: 100 });
        r.append(Vec2 { x: 150, y: 100 });

        assert_eq!(r.contains_point(Vec2 { x: 100., y: 50. }), true);
    }
}
