use crate::room::{Room, RoomId};

pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub(crate) fn new() -> Dungeon {
        Dungeon { rooms: vec![] }
    }

    pub(crate) fn add_room(&mut self, mut room: Room) -> RoomId {
        let room_id = match room.id {
            None => {
                let room_id = self.next_id();
                room_id
            }
            Some(x) => x,
        };
        room.id = Some(room_id);
        self.rooms.push(room);
        room_id
    }

    fn next_id(&self) -> RoomId {
        let max_id = self.rooms.iter().map(|r| r.id.unwrap_or(0)).max();
        match max_id {
            None => 1,
            Some(x) => x + 1,
        }
    }

    pub(crate) fn room(&mut self, room_id: RoomId) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|r| r.id == Some(room_id))
    }
}
