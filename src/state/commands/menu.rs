use crate::{
    room::RoomId,
    state::{events::StateEvent, StateCommand},
};

pub struct SelectRoomCommand {
    pub room_id: RoomId,
}

impl StateCommand for SelectRoomCommand {
    fn execute(&self, state: &mut crate::state::State) -> Vec<crate::state::events::StateEvent> {
        state.active_room_id = Some(self.room_id);
        println!("Active Room {}", state.active_room_id.unwrap());
        vec![StateEvent::ActiveRoomChanged]
    }
}
