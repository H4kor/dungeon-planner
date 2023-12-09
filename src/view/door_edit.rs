use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib;
use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

pub struct DoorEdit {
    pub widget: Box,
    name_input: Entry,
    notes_input: TextView,
}

impl DoorEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().build();
        let notes_i = TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .left_margin(10)
            .right_margin(10)
            .build();
        {
            let control = control.clone();
            name_i.connect_changed(move |field| {
                let name = field.text().to_string();
                if let Ok(mut control) = control.try_borrow_mut() {
                    match control.state.active_door_id {
                        None => (),
                        Some(door_id) => control.apply(StateCommand::ChangeDoorName(door_id, name)),
                    }
                }
            });
        }

        {
            let buffer = notes_i.buffer();
            let control = control.clone();
            buffer.connect_changed(move |buffer| {
                let (start, end) = buffer.bounds();
                let notes = buffer.text(&start, &end, true).to_string();
                let mut control = control.borrow_mut();
                match control.state.active_door_id {
                    None => (),
                    Some(door_id) => {
                        if let Some(door) = control.state.dungeon.door(door_id) {
                            if door.notes != notes {
                                control.apply(StateCommand::ChangeDoorNotes(door_id, notes))
                            };
                        }
                    }
                }
            });
            // notes_i.buffer().connect_end_user_action();
        }

        let b = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        b.append(&Label::new(Some("Name")));
        b.append(&name_i);
        b.append(&Label::new(Some("Notes")));
        b.append(&notes_i);

        b.set_visible(false);

        let re = Rc::new(RefCell::new(DoorEdit {
            widget: b,
            name_input: name_i,
            notes_input: notes_i,
        }));

        control.borrow_mut().subscribe_any(re.clone());

        re
    }
}

impl StateEventSubscriber for DoorEdit {
    fn on_state_event(
        &mut self,
        state: &mut crate::state::State,
        event: StateEvent,
    ) -> Vec<StateCommand> {
        match event {
            StateEvent::ActiveDoorChanged(None) => self.widget.set_visible(false),
            StateEvent::ActiveDoorChanged(Some(door_id)) => {
                let door = state.dungeon.door_mut(door_id).unwrap();
                self.name_input.set_text(&door.name);
                self.notes_input.buffer().set_text(&door.notes);
                self.widget.set_visible(true);
            }
            StateEvent::Reset => self.widget.set_visible(false),
            _ => (),
        }
        vec![]
    }
}
