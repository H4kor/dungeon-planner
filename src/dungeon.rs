use crate::room::Room;

pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub(crate) fn new() -> Dungeon {
        Dungeon { rooms: vec![] }
    }
}
