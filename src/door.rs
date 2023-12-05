use crate::{
    room::{RoomId, WallId},
    view::primitives::Primitive,
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

    pub fn draw() -> Vec<Box<dyn Primitive>> {
        vec![]
    }
}
