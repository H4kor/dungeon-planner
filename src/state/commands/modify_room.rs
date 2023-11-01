use crate::{
    common::Vec2,
    room::RoomId,
    state::{events::StateEvent, StateCommand},
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
}
