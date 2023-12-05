use crate::{
    common::{Rgb, Vec2},
    room::{RoomId, Wall, WallId},
    view::primitives::{Line, Primitive},
};

pub type DoorId = u32;

pub struct Door {
    pub id: Option<DoorId>,
    part_of: RoomId,
    leads_to: Option<RoomId>,
    width: f64,
    on_wall: WallId,
    position: f64,
}

impl Door {
    pub fn new(
        part_of: RoomId,
        leads_to: Option<RoomId>,
        width: f64,
        on_wall: WallId,
        position: f64,
    ) -> Self {
        Door {
            id: None,
            part_of: part_of,
            leads_to: leads_to,
            width: width,
            on_wall: on_wall,
            position: position,
        }
    }

    pub fn draw(&self, wall: &Wall) -> Vec<Box<dyn Primitive>> {
        let world_pos = wall.rel_to_world(self.position);
        let tangent = wall.tangent();

        vec![Box::new(Line {
            color: Rgb {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            from: world_pos - (self.width / 2.0) * tangent,
            to: world_pos + (self.width / 2.0) * tangent,
            width: 20.0,
        })]
    }
}
