use crate::{
    common::Vec2,
    room::{Room, RoomId},
};

use super::{events::StateEvent, State};

#[derive(Clone)]
pub enum StateCommand {
    AddRoom,
    SelectRoom(Option<RoomId>),
    AddVertexToRoom(RoomId, Vec2<i32>),
    ChangeRoomName(RoomId, String),
    ChangeRoomNotes(RoomId, String),
}

impl StateCommand {
    pub fn execute(&self, state: &mut State) -> Vec<StateEvent> {
        match self {
            StateCommand::AddRoom => {
                let room_id = state.dungeon.add_room(Room::new(None));
                vec![StateEvent::RoomAdded(room_id)]
            }
            StateCommand::SelectRoom(room_id) => {
                state.active_room_id = *room_id;
                vec![StateEvent::ActiveRoomChanged(*room_id)]
            }
            StateCommand::AddVertexToRoom(room_id, pos) => {
                state.dungeon.room(*room_id).unwrap().append(*pos);
                vec![StateEvent::RoomModified(*room_id)]
            }
            StateCommand::ChangeRoomName(room_id, name) => {
                state.dungeon.room(*room_id).unwrap().name = name.clone();
                vec![StateEvent::RoomModified(*room_id)]
            }
            StateCommand::ChangeRoomNotes(room_id, notes) => {
                state.dungeon.room(*room_id).unwrap().notes = notes.clone();
                vec![StateEvent::RoomModified(*room_id)]
            }
        }
    }
}
