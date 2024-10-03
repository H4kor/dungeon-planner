use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{prelude::*, Label, PolicyType, ScrolledWindow, TextView};
use gtk::{Box, Entry};

use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

pub struct DungeonEdit {
    pub widget: Box,
    name_input: Entry,
    notes_input: TextView,
}

impl DungeonEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().css_classes(vec!["form-input"]).build();
        let notes_i = TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .editable(true)
            .left_margin(10)
            .right_margin(10)
            .build();

        name_i.connect_changed(clone!(@strong control => move |field| {
            let name = field.text().to_string();
            if let Ok(mut control) = control.try_borrow_mut() {
                control.apply(StateCommand::ChangeDungeonName(name))
            }
        }));

        notes_i
            .buffer()
            .connect_changed(clone!(@strong control => move |buffer| {
                let (start, end) = buffer.bounds();
                let notes = buffer.text(&start, &end, true).to_string();
                if let Ok(mut control) = control.try_borrow_mut() {
                    control.apply(StateCommand::ChangeDungeonNotes(notes))
                }
            }));

        let b = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        b.append(&Label::new(Some("Name")));
        b.append(&name_i);
        b.append(&Label::new(Some("Notes")));
        b.append(
            &ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
                .min_content_width(360)
                .height_request(300)
                .child(&notes_i)
                .css_classes(vec!["form-input"])
                .build(),
        );

        let re = Rc::new(RefCell::new(DungeonEdit {
            widget: b,
            name_input: name_i,
            notes_input: notes_i,
        }));

        control.borrow_mut().subscribe_any(re.clone());

        re
    }

    fn sync(&mut self, state: &crate::state::State) {
        self.name_input.set_text(&state.dungeon.name);
        self.notes_input.buffer().set_text(&state.dungeon.notes);
    }
}

impl StateEventSubscriber for DungeonEdit {
    fn on_state_event(&mut self, state: &crate::state::State, event: StateEvent) {
        match event {
            StateEvent::DungeonModified => self.sync(state),
            StateEvent::Reset => self.sync(state),
            StateEvent::Reload => self.sync(state),
            _ => (),
        }
    }
}
