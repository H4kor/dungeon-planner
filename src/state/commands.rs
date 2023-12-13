use crate::{
    common::Vec2,
    door::{Door, DoorId},
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
    DeleteRoom(RoomId),
    AddDoor(Door),
    SelectDoor(Option<DoorId>),
    ChangeDoorName(DoorId, String),
    ChangeDoorNotes(DoorId, String),
    ChangeDoorLeadsTo(DoorId, Option<RoomId>),
    DeleteDoor(DoorId),
}

impl StateCommand {
    pub fn execute(&self, state: &mut State) -> Vec<StateEvent> {
        match self {
            StateCommand::AddRoom => {
                let room_id = state.dungeon.add_room(Room::new(None));
                state.active_room_id = Some(room_id);
                state.mode = EditMode::AppendRoom;
                vec![
                    StateEvent::RoomAdded(room_id),
                    StateEvent::ActiveRoomChanged(Some(room_id)),
                    StateEvent::EditModeChanged(EditMode::AppendRoom),
                ]
            }
            StateCommand::ChangeMode(mode) => {
                state.mode = *mode;
                vec![StateEvent::EditModeChanged(*mode)]
            }
            StateCommand::SelectRoom(room_id) => {
                state.active_room_id = *room_id;
                state.active_door_id = None;
                vec![
                    StateEvent::ActiveRoomChanged(*room_id),
                    StateEvent::ActiveDoorChanged(None),
                ]
            }
            StateCommand::SelectDoor(door_id) => {
                state.active_room_id = None;
                state.active_door_id = *door_id;
                vec![
                    StateEvent::ActiveRoomChanged(None),
                    StateEvent::ActiveDoorChanged(*door_id),
                ]
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
            StateCommand::DeleteRoom(room_id) => {
                let deleted_door_ids = state.dungeon.remove_room(*room_id);
                let mut events: Vec<StateEvent> = deleted_door_ids
                    .iter()
                    .map(|id| StateEvent::DoorDeleted(*id))
                    .collect();
                events.push(StateEvent::RoomDeleted(*room_id));

                if state.active_room_id == Some(*room_id) {
                    state.active_room_id = None;
                    events.push(StateEvent::ActiveRoomChanged(None));
                }
                events
            }
            StateCommand::AddDoor(door) => {
                let door_id = state.dungeon.add_door(door.clone());
                vec![
                    StateEvent::RoomModified(door.part_of),
                    StateEvent::DoorAdded(door_id),
                ]
            }
            StateCommand::ChangeDoorName(door_id, name) => {
                state.dungeon.door_mut(*door_id).unwrap().name = name.clone();
                vec![StateEvent::DoorModified(*door_id)]
            }
            StateCommand::ChangeDoorNotes(door_id, notes) => {
                state.dungeon.door_mut(*door_id).unwrap().notes = notes.clone();
                vec![StateEvent::DoorModified(*door_id)]
            }
            StateCommand::ChangeDoorLeadsTo(door_id, room_id) => {
                state.dungeon.door_mut(*door_id).unwrap().leads_to = *room_id;
                vec![StateEvent::DoorModified(*door_id)]
            }
            StateCommand::DeleteDoor(door_id) => {
                state.dungeon.remove_door(*door_id);
                let mut events = vec![StateEvent::DoorDeleted(*door_id)];
                if state.active_door_id == Some(*door_id) {
                    state.active_door_id = None;
                    events.push(StateEvent::ActiveDoorChanged(None));
                }
                events
            }
        }
    }
}
