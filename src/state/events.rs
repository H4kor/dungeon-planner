use crate::{chamber::ChamberId, door::DoorId, object::ObjectId};
use std::hash::Hash;

use super::EditMode;

#[derive(Clone, Debug)]
pub enum StateEvent {
    ChamberAdded(ChamberId),
    ChamberModified(ChamberId),
    ChamberDeleted(ChamberId),
    ActiveChamberChanged(Option<ChamberId>),
    ActiveDoorChanged(Option<DoorId>),
    ActiveObjectChanged(Option<ObjectId>),
    EditModeChanged(EditMode),
    DoorAdded(DoorId),
    DoorModified(DoorId),
    DoorDeleted(DoorId),
    ObjectAdded(ObjectId),
    ObjectDeleted(ObjectId),
    ObjectModified(ObjectId),
    DungeonModified,
    Reset,
    Reload,
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
        assert_eq!(
            StateEvent::ChamberAdded(10) == StateEvent::ChamberAdded(11),
            true
        );
        assert_eq!(
            StateEvent::ChamberAdded(10) == StateEvent::ChamberModified(10),
            false
        );
    }
}
