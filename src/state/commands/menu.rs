use crate::{
    room::RoomId,
    state::{events::StateEvent, StateCommand, StateCommandData},
};

pub struct SelectRoomCommand {
    pub room_id: Option<RoomId>,
}

impl StateCommand for SelectRoomCommand {
    fn execute(&self, state: &mut crate::state::State) -> Vec<crate::state::events::StateEvent> {
        state.active_room_id = self.room_id;
        vec![StateEvent::ActiveRoomChanged(self.room_id)]
    }

    fn data(&self) -> crate::state::StateCommandData {
        StateCommandData {
            name: "SelectRoomCommand".to_owned(),
            data: match self.room_id {
                Some(x) => format!("{}", x),
                None => "".to_owned(),
            },
        }
    }
}
