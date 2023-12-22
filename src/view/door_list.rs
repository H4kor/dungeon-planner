use crate::state::{
    events::StateEvent, State, StateCommand, StateController, StateEventSubscriber,
};
use crate::view::door_list_entry::DoorListEntry;
use cairo::glib::{clone, Propagation};
use gtk::{gdk, prelude::*, EventControllerKey};
use gtk::{ListBox, PolicyType, ScrolledWindow};
use std::{cell::RefCell, rc::Rc};

pub struct DoorList {
    pub list_box: ListBox,
    pub scrolled_window: ScrolledWindow,
    pub rows: Vec<DoorListEntry>,
}

impl DoorList {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let list_box = ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .build();
        let door_list = Rc::new(RefCell::new(DoorList {
            list_box: list_box.clone(),
            scrolled_window: ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
                .min_content_width(360)
                .height_request(300)
                .child(&list_box)
                .build(),
            rows: vec![],
        }));

        list_box.connect_row_activated(clone!(@strong control => move |_, row| {
            let door_id = row
                .clone()
                .dynamic_cast::<DoorListEntry>()
                .unwrap()
                .door_id();
            control
                .borrow_mut()
                .apply(StateCommand::SelectDoor(Some(door_id)));
        }));

        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(
            clone!(@strong control, @strong list_box => move |_, key, _, _| {
                match key {
                    gdk::Key::Delete => {
                        if let Some(row) = list_box.selected_row() {
                            let door_id = row
                                .clone()
                                .dynamic_cast::<DoorListEntry>()
                                .unwrap()
                                .door_id();
                            control
                                .borrow_mut()
                                .apply(StateCommand::DeleteDoor(door_id))
                        }
                    },
                    _ => (),
                }
                Propagation::Proceed
            }),
        );
        list_box.add_controller(key_controller);

        let mut state = control.borrow_mut();
        state.subscribe_any(door_list.clone());
        door_list
    }

    fn rebuild_list(&mut self, state: &State) {
        for row in &self.rows {
            self.list_box.remove(row)
        }
        self.rows = vec![];
        for door in &state.dungeon.doors {
            let door_label = DoorListEntry::new(&door.clone());
            self.rows.push(door_label);
            self.list_box.append(self.rows.last().unwrap());
        }
        match state.active_door_id {
            None => self.list_box.unselect_all(),
            Some(door_id) => self
                .list_box
                .select_row(self.rows.iter().find(|r| r.door_id() == door_id)),
        }
    }
}

impl StateEventSubscriber for DoorList {
    fn on_state_event(&mut self, state: &mut State, event: StateEvent) -> Vec<StateCommand> {
        match event {
            StateEvent::DoorAdded(_) => {
                self.rebuild_list(state);
            }
            StateEvent::DoorModified(door_id) => {
                let door = state.dungeon.door_mut(door_id).unwrap();
                self.rows
                    .iter_mut()
                    .filter(|r| r.door_id() == door_id)
                    .for_each(|w| w.update(door));
            }
            StateEvent::ActiveDoorChanged(door_id) => match door_id {
                None => self.list_box.unselect_all(),
                Some(door_id) => self
                    .list_box
                    .select_row(self.rows.iter().find(|r| r.door_id() == door_id)),
            },
            StateEvent::Reset => self.rebuild_list(state),
            StateEvent::DoorDeleted(_) => self.rebuild_list(state),
            _ => (),
        }
        vec![]
    }
}
