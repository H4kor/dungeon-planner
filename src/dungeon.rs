use crate::{
    common::Vec2,
    door::{Door, DoorId},
    room::{Room, RoomId, Wall},
};

/// A Dungeon is the main object we care about
/// It consists of multiple rooms
pub struct Dungeon {
    pub rooms: Vec<Room>,
    pub doors: Vec<Door>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            rooms: vec![],
            doors: vec![],
        }
    }

    /// Add a room to the dungeon
    /// If the room does not have an id yet, it is generate before insertion
    ///
    /// Returns the `RoomId`
    pub fn add_room(&mut self, mut room: Room) -> RoomId {
        let room_id = match room.id {
            None => self.next_room_id(),
            Some(x) => {
                // if Id is already used, generate a new one.
                match self.room_mut(x) {
                    Some(_) => self.next_room_id(),
                    None => x,
                }
            }
        };
        room.id = Some(room_id);
        self.rooms.push(room);
        room_id
    }

    /// generates an unused `RoomId`.
    fn next_room_id(&self) -> RoomId {
        let max_id = self.rooms.iter().map(|r| r.id.unwrap_or(0)).max();
        match max_id {
            None => 1,
            Some(x) => x + 1,
        }
    }

    /// get a room by its id
    pub fn room_mut(&mut self, room_id: RoomId) -> Option<&mut Room> {
        self.rooms.iter_mut().find(|r| r.id == Some(room_id))
    }
    pub fn room(&self, room_id: RoomId) -> Option<&Room> {
        self.rooms.iter().find(|r| r.id == Some(room_id))
    }

    /// get the nearest wall to a given point
    /// Returns the RoomId and WallId
    pub fn nearest_wall(&self, pos: Vec2<f64>) -> Option<(RoomId, Wall)> {
        let mut min_room_id = None;
        let mut min_wall = None;
        let mut min_d = f64::INFINITY;
        for room in self.rooms.iter() {
            if let Some(wall) = room.nearest_wall(pos) {
                let d = wall.distance(pos);
                if d < min_d {
                    min_room_id = room.id;
                    min_wall = Some(wall);
                    min_d = d;
                }
            }
        }
        match min_room_id {
            None => None,
            Some(room_id) => Some((room_id, min_wall.unwrap())),
        }
    }

    pub(crate) fn room_at(&self, pos: Vec2<f64>) -> Option<RoomId> {
        for room in &self.rooms {
            if room.contains_point(pos.into()) {
                return room.id;
            }
        }
        None
    }

    pub(crate) fn rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    pub(crate) fn remove_room(&mut self, room_id: u32) {
        let idx = self.rooms.iter().position(|r| r.id == Some(room_id));
        match idx {
            Some(i) => {
                self.rooms.remove(i);
            }
            None => {
                println!("Room Id not found for deletion")
            }
        };
    }

    pub fn door(&self, id: DoorId) -> Option<&Door> {
        self.doors.iter().find(|d| d.id == Some(id))
    }

    pub fn add_door(&mut self, mut door: Door) -> DoorId {
        let door_id = match door.id {
            None => self.next_door_id(),
            Some(x) => {
                // if Id is already used, generate a new one.
                match self.door(x) {
                    Some(_) => self.next_door_id(),
                    None => x,
                }
            }
        };
        door.id = Some(door_id);
        self.doors.push(door);
        door_id
    }

    fn next_door_id(&self) -> DoorId {
        self.doors
            .iter()
            .map(|r| r.id.unwrap_or(0))
            .max()
            .unwrap_or(0)
            + 1
    }
}
