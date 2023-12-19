use crate::{
    chamber::{Chamber, ChamberId, WallId},
    common::Vec2,
    door::{Door, DoorId},
};

use super::{events::StateEvent, EditMode, State};

#[derive(Clone)]
pub enum StateCommand {
    AddChamber,
    ChangeMode(EditMode),
    SelectChamber(Option<ChamberId>),
    AddVertexToChamber(ChamberId, Vec2<i32>),
    ChangeChamberName(ChamberId, String),
    ChangeChamberNotes(ChamberId, String),
    ChangeChamberHidden(ChamberId, bool),
    SplitWall(ChamberId, WallId, Vec2<i32>),
    DeleteChamber(ChamberId),
    AddDoor(Door),
    SelectDoor(Option<DoorId>),
    ChangeDoorName(DoorId, String),
    ChangeDoorNotes(DoorId, String),
    ChangeDoorLeadsTo(DoorId, Option<ChamberId>),
    DeleteDoor(DoorId),
}

impl StateCommand {
    pub fn execute(&self, state: &mut State) -> Vec<StateEvent> {
        match self {
            StateCommand::AddChamber => {
                let chamber_id = state.dungeon.add_chamber(Chamber::new());
                state.active_chamber_id = Some(chamber_id);
                state.mode = EditMode::AppendChamber;
                vec![
                    StateEvent::ChamberAdded(chamber_id),
                    StateEvent::ActiveChamberChanged(Some(chamber_id)),
                    StateEvent::EditModeChanged(EditMode::AppendChamber),
                ]
            }
            StateCommand::ChangeMode(mode) => {
                state.mode = *mode;
                vec![StateEvent::EditModeChanged(*mode)]
            }
            StateCommand::SelectChamber(chamber_id) => {
                state.active_chamber_id = *chamber_id;
                state.active_door_id = None;
                vec![
                    StateEvent::ActiveChamberChanged(*chamber_id),
                    StateEvent::ActiveDoorChanged(None),
                ]
            }
            StateCommand::SelectDoor(door_id) => {
                state.active_chamber_id = None;
                state.active_door_id = *door_id;
                vec![
                    StateEvent::ActiveChamberChanged(None),
                    StateEvent::ActiveDoorChanged(*door_id),
                ]
            }
            StateCommand::AddVertexToChamber(chamber_id, pos) => {
                state.dungeon.chamber_mut(*chamber_id).unwrap().append(*pos);
                vec![StateEvent::ChamberModified(*chamber_id)]
            }
            StateCommand::ChangeChamberName(chamber_id, name) => {
                state.dungeon.chamber_mut(*chamber_id).unwrap().name = name.clone();
                vec![StateEvent::ChamberModified(*chamber_id)]
            }
            StateCommand::ChangeChamberNotes(chamber_id, notes) => {
                state.dungeon.chamber_mut(*chamber_id).unwrap().notes = notes.clone();
                vec![StateEvent::ChamberModified(*chamber_id)]
            }
            StateCommand::ChangeChamberHidden(chamber_id, hidden) => {
                state.dungeon.chamber_mut(*chamber_id).unwrap().hidden = *hidden;
                vec![StateEvent::ChamberModified(*chamber_id)]
            }
            StateCommand::SplitWall(chamber_id, wall_id, pos) => {
                state
                    .dungeon
                    .chamber_mut(*chamber_id)
                    .unwrap()
                    .split(*wall_id, *pos);
                vec![StateEvent::ChamberModified(*chamber_id)]
            }
            StateCommand::DeleteChamber(chamber_id) => {
                let deleted_door_ids = state.dungeon.remove_chamber(*chamber_id);
                let mut events: Vec<StateEvent> = deleted_door_ids
                    .iter()
                    .map(|id| StateEvent::DoorDeleted(*id))
                    .collect();
                events.push(StateEvent::ChamberDeleted(*chamber_id));

                if state.active_chamber_id == Some(*chamber_id) {
                    state.active_chamber_id = None;
                    events.push(StateEvent::ActiveChamberChanged(None));
                }
                events
            }
            StateCommand::AddDoor(door) => {
                let mut door = door.clone();
                let wall = state
                    .dungeon
                    .chamber(door.part_of)
                    .unwrap()
                    .wall(door.on_wall)
                    .unwrap();
                let world_pos = wall.rel_to_world(door.position);
                // check if a "leads_to" can be uniquely determined
                // iterate all walls
                let containing_walls = state
                    .dungeon
                    .walls()
                    .iter()
                    .filter(|w| {
                        // skip all wals of chamber containing the door
                        // check if world pos of door lies on wall
                        w.chamber_id != door.part_of && w.distance(world_pos) < 1e-6
                    })
                    .map(|w| w.chamber_id)
                    .collect::<std::collections::HashSet<_>>();

                if containing_walls.len() == 1 {
                    door.leads_to = Some(*containing_walls.iter().next().unwrap());
                }
                let door_id = state.dungeon.add_door(door.clone());

                vec![
                    StateEvent::ChamberModified(door.part_of),
                    StateEvent::DoorAdded(door_id),
                ]
            }
            StateCommand::ChangeDoorName(door_id, name) => {
                state.dungeon.door_mut(*door_id).unwrap().name = name.clone();
                vec![StateEvent::DoorModified(*door_id)]
            }
            StateCommand::ChangeDoorNotes(door_id, notes) => {
                state.dungeon.door_mut(*door_id).unwrap().notes = notes.clone();
                vec![StateEvent::DoorModified(*door_id)]
            }
            StateCommand::ChangeDoorLeadsTo(door_id, chamber_id) => {
                state.dungeon.door_mut(*door_id).unwrap().leads_to = *chamber_id;
                vec![StateEvent::DoorModified(*door_id)]
            }
            StateCommand::DeleteDoor(door_id) => {
                state.dungeon.remove_door(*door_id);
                let mut events = vec![StateEvent::DoorDeleted(*door_id)];
                if state.active_door_id == Some(*door_id) {
                    state.active_door_id = None;
                    events.push(StateEvent::ActiveDoorChanged(None));
                }
                events
            }
        }
    }
}
