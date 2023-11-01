use crate::{
    room::Room,
    state::{events::StateEvent, State, StateCommand},
};

pub struct AddRoomCommand {
    room: Room,
}

impl AddRoomCommand {
    pub fn new(room: Room) -> Self {
        AddRoomCommand { room: room }
    }
}

impl StateCommand for AddRoomCommand {
    fn execute(&self, state: &mut State) -> Vec<StateEvent> {
        let room_id = state.dungeon.add_room(self.room.clone());
        vec![StateEvent::RoomAdded(room_id)]
    }
}
