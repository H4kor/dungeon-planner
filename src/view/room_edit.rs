use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::state::commands::{ChangeRoomName, ChangeRoomNotes};
use crate::state::StateController;

pub struct RoomEdit {
    pub widget: Box,
    name_input: Entry,
    notes_input: TextView,
}

impl RoomEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let name_i = Entry::builder().build();
        let notes_i = TextView::builder().build();
        {
            let control = control.clone();
            name_i.connect_changed(move |field| {
                let name = field.text().to_string();
                let mut control = control.borrow_mut();
                match control.state.active_room_id {
                    None => (),
                    Some(room_id) => control.apply(std::boxed::Box::new(ChangeRoomName {
                        room_id: room_id,
                        name: name,
                    })),
                }
            });
        }

        {
            let control = control.clone();
            notes_i.buffer().connect_changed(move |buffer| {
                let notes = buffer.to_string();
                let mut control = control.borrow_mut();
                match control.state.active_room_id {
                    None => (),
                    Some(room_id) => control.apply(std::boxed::Box::new(ChangeRoomNotes {
                        room_id: room_id,
                        notes: notes,
                    })),
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

        RoomEdit {
            widget: b,
            name_input: name_i,
            notes_input: notes_i,
        }
    }
}
