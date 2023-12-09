use crate::{door::DoorId, room::RoomId};
use std::hash::Hash;

use super::EditMode;

#[derive(Clone, Debug)]
pub enum StateEvent {
    RoomAdded(RoomId),
    RoomModified(RoomId),
    RoomDeleted(RoomId),
    ActiveRoomChanged(Option<RoomId>),
    ActiveDoorChanged(Option<DoorId>),
    EditModeChanged(EditMode),
    DoorAdded(DoorId),
    DoorModified(DoorId),
    Reset,
}

impl PartialEq for StateEvent {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
impl Eq for StateEvent {}

impl Hash for StateEvent {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::state::events::StateEvent;

    #[test]
    fn event_eq() {
        assert_eq!(StateEvent::RoomAdded(10) == StateEvent::RoomAdded(11), true);
        assert_eq!(
            StateEvent::RoomAdded(10) == StateEvent::RoomModified(10),
            false
        );
    }
}
