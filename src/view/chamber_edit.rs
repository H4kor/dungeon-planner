use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{prelude::*, CheckButton, Label, TextView};
use gtk::{Box, Entry};

use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

pub struct ChamberEdit {
    pub widget: Box,
    name_input: Entry,
    notes_input: TextView,
}

impl ChamberEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().build();
        let notes_i = TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .left_margin(10)
            .right_margin(10)
            .build();
        let hidden_i = CheckButton::builder().label("Hidden").build();

        name_i.connect_changed(clone!(@strong control => move |field| {
            let name = field.text().to_string();
            if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_chamber_id {
                    None => (),
                    Some(chamber_id) => control.apply(StateCommand::ChangeChamberName(chamber_id, name)),
                }
            }
        }));

        notes_i
            .buffer()
            .connect_changed(clone!(@strong control => move |buffer| {
                let (start, end) = buffer.bounds();
                let notes = buffer.text(&start, &end, true).to_string();
                if let Ok(mut control) = control.try_borrow_mut() {
                    match control.state.active_chamber_id {
                        None => (),
                        Some(chamber_id) => {
                            if let Some(chamber) = control.state.dungeon.chamber(chamber_id) {
                                if chamber.notes != notes {
                                    control.apply(StateCommand::ChangeChamberNotes(chamber_id, notes))
                                };
                            }
                        }
                    }
                }
            }));

        hidden_i.connect_toggled(
            clone!(@strong control => move |w| if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_chamber_id {
                    None => (),
                    Some(chamber_id) => control.apply(StateCommand::ChangeChamberHidden(chamber_id, w.is_active())),
                }
            }),
        );

        let b = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        b.append(&Label::new(Some("Name")));
        b.append(&name_i);
        b.append(&hidden_i);
        b.append(&Label::new(Some("Notes")));
        b.append(&notes_i);

        b.set_visible(false);

        let re = Rc::new(RefCell::new(ChamberEdit {
            widget: b,
            name_input: name_i,
            notes_input: notes_i,
        }));

        control.borrow_mut().subscribe_any(re.clone());

        re
    }
}

impl StateEventSubscriber for ChamberEdit {
    fn on_state_event(
        &mut self,
        state: &mut crate::state::State,
        event: StateEvent,
    ) -> Vec<StateCommand> {
        match event {
            StateEvent::ActiveChamberChanged(None) => self.widget.set_visible(false),
            StateEvent::ActiveChamberChanged(Some(chamber_id)) => {
                let chamber = state.dungeon.chamber_mut(chamber_id).unwrap();
                self.name_input.set_text(&chamber.name);
                self.notes_input.buffer().set_text(&chamber.notes);
                self.widget.set_visible(true);
            }
            StateEvent::Reset => self.widget.set_visible(false),
            _ => (),
        }
        vec![]
    }
}
