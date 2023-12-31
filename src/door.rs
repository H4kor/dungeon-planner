use crate::{
    chamber::{ChamberId, Wall, WallId},
    common::{Rgb, Vec2},
    view::primitives::{Line, Primitive},
};

pub type DoorId = u32;

pub struct DoorDrawOptions {
    pub color: Option<Rgb>,
}

impl DoorDrawOptions {
    pub(crate) fn empty() -> DoorDrawOptions {
        DoorDrawOptions { color: None }
    }
}

#[derive(Clone, Debug)]
pub struct Door {
    pub id: DoorId,
    pub name: String,
    pub notes: String,
    pub hidden: bool,
    pub part_of: ChamberId,
    pub leads_to: Option<ChamberId>,
    pub width: f64,
    pub on_wall: WallId,
    pub position: f64,
}

impl Door {
    pub fn new(
        part_of: ChamberId,
        leads_to: Option<ChamberId>,
        width: f64,
        on_wall: WallId,
        position: f64,
    ) -> Self {
        Door {
            id: 1,
            name: "".to_owned(),
            notes: "".to_owned(),
            hidden: false,
            part_of: part_of,
            leads_to: leads_to,
            width: width,
            on_wall: on_wall,
            position: position,
        }
    }

    pub fn draw(&self, wall: &Wall, options: DoorDrawOptions) -> Vec<Box<dyn Primitive>> {
        let world_pos = wall.rel_to_world(self.position);
        let tangent = wall.tangent();

        let color = options.color.unwrap_or(Rgb {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        });

        // TODO: better door visuals
        if self.hidden {
            vec![
                Box::new(Line {
                    color: color,
                    from: world_pos - (self.width / 2.0) * tangent,
                    to: world_pos + (self.width / 2.0) * tangent,
                    width: 10.0,
                    dashed: false,
                }),
                Box::new(Line {
                    color: color,
                    from: world_pos - (self.width / 4.0) * tangent,
                    to: world_pos + (self.width / 4.0) * tangent,
                    width: 20.0,
                    dashed: false,
                }),
            ]
        } else {
            vec![Box::new(Line {
                color: color,
                from: world_pos - (self.width / 2.0) * tangent,
                to: world_pos + (self.width / 2.0) * tangent,
                width: 20.0,
                dashed: false,
            })]
        }
    }

    pub(crate) fn contains_point(&self, wall: &Wall, pos: Vec2<f64>) -> bool {
        (pos - wall.rel_to_world(self.position)).len() < self.width
    }
}
