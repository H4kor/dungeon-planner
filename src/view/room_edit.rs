use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

pub struct RoomEdit {
    pub widget: Box,
    name_input: Entry,
    notes_input: TextView,
}

impl RoomEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().build();
        let notes_i = TextView::builder().build();
        {
            let control = control.clone();
            name_i.connect_changed(move |field| {
                let name = field.text().to_string();
                if let Ok(mut control) = control.try_borrow_mut() {
                    match control.state.active_room_id {
                        None => (),
                        Some(room_id) => control.apply(StateCommand::ChangeRoomName(room_id, name)),
                    }
                }
            });
        }

        {
            let control = control.clone();
            notes_i.buffer().connect_end_user_action(move |buffer| {
                let (start, end) = buffer.bounds();
                let notes = buffer.text(&start, &end, true).to_string();
                let mut control = control.borrow_mut();
                match control.state.active_room_id {
                    None => (),
                    Some(room_id) => control.apply(StateCommand::ChangeRoomNotes(room_id, notes)),
                }
            });
        }

        let b = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        b.append(&Label::new(Some("Name")));
        b.append(&name_i);
        b.append(&Label::new(Some("Notes")));
        b.append(&notes_i);

        b.set_visible(false);

        let re = Rc::new(RefCell::new(RoomEdit {
            widget: b,
            name_input: name_i,
            notes_input: notes_i,
        }));

        control
            .borrow_mut()
            .subscribe(StateEvent::ActiveRoomChanged(None), re.clone());

        re
    }
}

impl StateEventSubscriber for RoomEdit {
    fn on_state_event(
        &mut self,
        state: &mut crate::state::State,
        event: StateEvent,
    ) -> Vec<StateCommand> {
        match event {
            StateEvent::ActiveRoomChanged(None) => self.widget.set_visible(false),
            StateEvent::ActiveRoomChanged(Some(room_id)) => {
                let room = state.dungeon.room_mut(room_id).unwrap();
                self.name_input.set_text(&room.name);
                self.notes_input.buffer().set_text(&room.notes);
                self.widget.set_visible(true);
            }
            _ => (),
        }
        vec![]
    }
}
