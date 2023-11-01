use crate::{
    common::Vec2,
    room::RoomId,
    state::{events::StateEvent, StateCommand},
};

pub struct AddVertexToRoomCommand {
    room_id: RoomId,
    pos: Vec2<i32>,
}

impl AddVertexToRoomCommand {
    pub fn new(room_id: RoomId, pos: Vec2<i32>) -> Self {
        AddVertexToRoomCommand {
            room_id: room_id,
            pos: pos,
        }
    }
}

impl StateCommand for AddVertexToRoomCommand {
    fn execute(&self, state: &mut crate::state::State) -> Vec<crate::state::events::StateEvent> {
        state.dungeon.room(self.room_id).unwrap().append(self.pos);
        vec![StateEvent::RoomModified(self.room_id)]
    }
}
