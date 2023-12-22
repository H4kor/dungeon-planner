use crate::observers::HistoryObserver;
use crate::state::{EditMode, StateCommand, StateController};
use cairo::glib::clone;
use gtk::gio::{ActionEntry, SimpleActionGroup};
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

fn change_mode_action(
    control: Rc<RefCell<StateController>>,
    mode: EditMode,
    name: &str,
) -> ActionEntry<SimpleActionGroup> {
    ActionEntry::builder(name)
        .activate(
            clone!(@strong control => move |_window: &SimpleActionGroup, _, _| {
                control.borrow_mut().apply(StateCommand::ChangeMode(mode));
            }),
        )
        .build()
}

pub fn edit_actions(
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
) -> SimpleActionGroup {
    let edit_actions = SimpleActionGroup::new();

    let edit_action_unselect = ActionEntry::builder("unselect")
        .activate(
            clone!(@strong control => move |_window: &SimpleActionGroup, _, _| {
                control.borrow_mut().apply(StateCommand::SelectChamber(None));
                control.borrow_mut().apply(StateCommand::ChangeMode(EditMode::Select))
            }),
        )
        .build();

    let edit_action_undo = ActionEntry::builder("undo")
        .activate(
            clone!(@strong control, @strong history => move |_window: &SimpleActionGroup, _, _| {
                control.borrow_mut().reset();
                let cmds = {
                    history.borrow_mut().undo();
                    history.borrow_mut().get_stack()
                };

                for cmd in cmds {
                    control.borrow_mut().apply_silent(cmd)
                }
                control.borrow_mut().reload();
            }),
        )
        .build();

    let edit_action_delete = ActionEntry::builder("delete_selected")
        .activate(
            clone!(@strong control => move |_window: &SimpleActionGroup, _, _| {
                let active_chamber_id = control.borrow().state.active_chamber_id;
                if let Some(chamber_id) = active_chamber_id {
                    control.borrow_mut().apply(StateCommand::DeleteChamber(chamber_id))
                } else {
                    let active_door_id = control.borrow().state.active_door_id;
                    if let Some(door_id) = active_door_id {
                        control.borrow_mut().apply(StateCommand::DeleteDoor(door_id));
                    }
                }
            }),
        )
        .build();

    let edit_action_add_chamber = ActionEntry::builder("add_chamber")
        .activate(
            clone!(@strong control => move |_window: &SimpleActionGroup, _, _| {
                control.borrow_mut().apply(StateCommand::AddChamber);
            }),
        )
        .build();

    edit_actions.add_action_entries([
        edit_action_unselect,
        edit_action_undo,
        edit_action_delete,
        edit_action_add_chamber,
        change_mode_action(control.clone(), EditMode::Select, "mode_select"),
        change_mode_action(
            control.clone(),
            EditMode::AppendChamber,
            "mode_append_chamber",
        ),
        change_mode_action(control.clone(), EditMode::SplitEdge, "mode_split_edge"),
        change_mode_action(control.clone(), EditMode::AddDoor, "mode_add_door"),
    ]);

    edit_actions
}
