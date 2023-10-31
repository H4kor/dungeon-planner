use crate::{
    room::Room,
    view::{grid::Grid, View},
};

pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub(crate) fn new() -> Dungeon {
        Dungeon { rooms: vec![] }
    }
}
