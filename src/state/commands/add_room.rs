use crate::{
    room::Room,
    state::{events::StateEvent, State, StateCommand, StateCommandData},
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

    fn data(&self) -> crate::state::StateCommandData {
        StateCommandData {
            name: "AddRoomCommand".to_owned(),
            data: "".to_owned(),
        }
    }
}
