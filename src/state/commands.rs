use crate::{
    common::Vec2,
    room::{Room, RoomId, WallId},
};

use super::{events::StateEvent, EditMode, State};

#[derive(Clone)]
pub enum StateCommand {
    AddRoom,
    ChangeMode(EditMode),
    SelectRoom(Option<RoomId>),
    AddVertexToRoom(RoomId, Vec2<i32>),
    ChangeRoomName(RoomId, String),
    ChangeRoomNotes(RoomId, String),
    SplitWall(RoomId, WallId, Vec2<i32>),
}

impl StateCommand {
    pub fn execute(&self, state: &mut State) -> Vec<StateEvent> {
        match self {
            StateCommand::AddRoom => {
                let room_id = state.dungeon.add_room(Room::new(None));
                vec![StateEvent::RoomAdded(room_id)]
            }
            StateCommand::ChangeMode(mode) => {
                state.mode = *mode;
                vec![StateEvent::EditModeChanged(*mode)]
            }
            StateCommand::SelectRoom(room_id) => {
                state.active_room_id = *room_id;
                vec![StateEvent::ActiveRoomChanged(*room_id)]
            }
            StateCommand::AddVertexToRoom(room_id, pos) => {
                state.dungeon.room_mut(*room_id).unwrap().append(*pos);
                vec![StateEvent::RoomModified(*room_id)]
            }
            StateCommand::ChangeRoomName(room_id, name) => {
                state.dungeon.room_mut(*room_id).unwrap().name = name.clone();
                vec![StateEvent::RoomModified(*room_id)]
            }
            StateCommand::ChangeRoomNotes(room_id, notes) => {
                state.dungeon.room_mut(*room_id).unwrap().notes = notes.clone();
                vec![StateEvent::RoomModified(*room_id)]
            }
            StateCommand::SplitWall(room_id, wall_id, pos) => {
                state
                    .dungeon
                    .room_mut(*room_id)
                    .unwrap()
                    .split(*wall_id, *pos);
                vec![StateEvent::RoomModified(*room_id)]
            }
        }
    }
}
