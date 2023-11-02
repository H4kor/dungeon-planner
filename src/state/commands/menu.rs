use crate::{
    room::RoomId,
    state::{events::StateEvent, StateCommand},
};

pub struct SelectRoomCommand {
    pub room_id: Option<RoomId>,
}

impl StateCommand for SelectRoomCommand {
    fn execute(&self, state: &mut crate::state::State) -> Vec<crate::state::events::StateEvent> {
        state.active_room_id = self.room_id;
        vec![StateEvent::ActiveRoomChanged(self.room_id)]
    }
}
