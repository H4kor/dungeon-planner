use crate::room::{RoomId, WallId};

pub struct Door {
    part_of: RoomId,
    leads_to: Option<RoomId>,
    width: f64,
    on_wall: WallId,
    position: f64,
}
