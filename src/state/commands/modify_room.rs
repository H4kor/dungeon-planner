use crate::{
    common::Vec2,
    room::RoomId,
    state::{events::StateEvent, StateCommand, StateCommandData},
};

pub struct AddVertexToRoomCommand {
    pub room_id: RoomId,
    pub pos: Vec2<i32>,
}

impl StateCommand for AddVertexToRoomCommand {
    fn execute(&self, state: &mut crate::state::State) -> Vec<crate::state::events::StateEvent> {
        state.dungeon.room(self.room_id).unwrap().append(self.pos);
        vec![StateEvent::RoomModified(self.room_id)]
    }

    fn data(&self) -> crate::state::StateCommandData {
        StateCommandData {
            name: "AddVertexToRoomCommand".to_owned(),
            data: format!("{} {} {}", self.room_id, self.pos.x, self.pos.y),
        }
    }
}

pub struct ChangeRoomName {
    pub room_id: RoomId,
    pub name: String,
}

impl StateCommand for ChangeRoomName {
    fn execute(&self, state: &mut crate::state::State) -> Vec<StateEvent> {
        state.dungeon.room(self.room_id).unwrap().name = self.name.clone();
        vec![StateEvent::RoomModified(self.room_id)]
    }

    fn data(&self) -> crate::state::StateCommandData {
        StateCommandData {
            name: "ChangeRoomName".to_owned(),
            data: format!("{} {}", self.room_id, self.name),
        }
    }
}

pub struct ChangeRoomNotes {
    pub room_id: RoomId,
    pub notes: String,
}

impl StateCommand for ChangeRoomNotes {
    fn execute(&self, state: &mut crate::state::State) -> Vec<StateEvent> {
        state.dungeon.room(self.room_id).unwrap().notes = self.notes.clone();
        vec![StateEvent::RoomModified(self.room_id)]
    }

    fn data(&self) -> crate::state::StateCommandData {
        StateCommandData {
            name: "ChangeRoomNotes".to_owned(),
            data: format!("{} {}", self.room_id, self.notes),
        }
    }
}
