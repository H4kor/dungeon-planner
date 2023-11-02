use crate::{
    room::Room,
    state::{events::StateEvent, State, StateCommand, StateCommandData},
};

pub struct AddRoomCommand {}

impl StateCommand for AddRoomCommand {
    fn execute(&self, state: &mut State) -> Vec<StateEvent> {
        let room_id = state.dungeon.add_room(Room::new(None));
        vec![StateEvent::RoomAdded(room_id)]
    }

    fn data(&self) -> crate::state::StateCommandData {
        StateCommandData {
            name: "AddRoomCommand".to_owned(),
            data: serde_json::Value::Null,
        }
    }
}
