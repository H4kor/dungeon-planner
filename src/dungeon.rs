use crate::{
    common::Vec2,
    room::{Room, RoomId, Wall},
};

/// A Dungeon is the main object we care about
/// It consists of multiple rooms
pub struct Dungeon {
    pub rooms: Vec<Room>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon { rooms: vec![] }
    }

    /// Add a room to the dungeon
    /// If the room does not have an id yet, it is generate before insertion
    ///
    /// Returns the `RoomId`
    pub fn add_room(&mut self, mut room: Room) -> RoomId {
        let room_id = match room.id {
            None => self.next_id(),
            Some(x) => {
                // if Id is already used, generate a new one.
                match self.room(x) {
                    Some(_) => self.next_id(),
                    None => x,
                }
            }
        };
        room.id = Some(room_id);
        self.rooms.push(room);
        room_id
    }

    /// generates an unused `RoomId`.
    fn next_id(&self) -> RoomId {
        let max_id = self.rooms.iter().map(|r| r.id.unwrap_or(0)).max();
        match max_id {
            None => 1,
            Some(x) => x + 1,
        }
    }

    /// get a room by its id
    pub fn room(&mut self, room_id: RoomId) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|r| r.id == Some(room_id))
    }

    /// get the nearest wall to a given point
    /// Returns the RoomId and WallId
    pub fn nearest_wall(&self, pos: Vec2<f64>) -> Option<(RoomId, Wall)> {
        let mut min_room_id = None;
        let mut min_wall = None;
        let mut min_d = f64::INFINITY;
        for room in self.rooms.iter() {
            for wall in room.walls().iter() {
                let d = wall.distance(pos);
                if d < min_d {
                    min_room_id = room.id;
                    min_wall = Some(*wall);
                    min_d = d;
                }
            }
        }
        match min_room_id {
            None => None,
            Some(room_id) => Some((room_id, min_wall.unwrap())),
        }
    }
}
