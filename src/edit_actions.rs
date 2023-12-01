use crate::export::to_pdf;
use crate::observers::HistoryObserver;
use crate::state::{StateCommand, StateController};
use crate::storage;
use cairo::glib::clone;
use gtk::gio::{ActionEntry, SimpleActionGroup};
use gtk::glib;
use gtk::{prelude::*, FileChooserDialog};
use std::cell::RefCell;
use std::rc::Rc;

pub fn edit_actions(
    control: Rc<RefCell<StateController>>,
    history: Rc<RefCell<HistoryObserver>>,
) -> SimpleActionGroup {
    let edit_actions = SimpleActionGroup::new();

    let edit_action_unselect = ActionEntry::builder("unselect")
        .activate(
            clone!(@strong control => move |_window: &SimpleActionGroup, _, _| {
                control.borrow_mut().apply(StateCommand::SelectRoom(None))
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

                history.borrow_mut().start_restore();
                for cmd in cmds {
                    control.borrow_mut().apply(cmd)
                }
                history.borrow_mut().end_restore();
            }),
        )
        .build();

    edit_actions.add_action_entries([edit_action_unselect, edit_action_undo]);

    edit_actions
}
