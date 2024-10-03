use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{gio, CheckButton, ListItem, PolicyType, ScrolledWindow, SignalListItemFactory};
use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::object::ObjectStyle;
use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

use super::chamber_list_object::ChamberObject;

pub struct ObjectEdit {
    pub widget: Box,
    part_of_label: Label,
    name_input: Entry,
    notes_input: TextView,
    hidden_input: CheckButton,
    blocker_style: CheckButton,
    stair_style: CheckButton,
    round_style: CheckButton,
}

impl ObjectEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().css_classes(vec!["form-input"]).build();
        let notes_i = TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .left_margin(10)
            .right_margin(10)
            .build();
        let hidden_i = CheckButton::builder()
            .css_classes(vec!["form-input"])
            .label("Hidden")
            .build();
        let blocker_style = CheckButton::builder().label("Blocker").build();
        let stair_style = CheckButton::builder()
            .label("Stairs")
            .group(&blocker_style)
            .build();
        let round_style = CheckButton::builder()
            .css_classes(vec!["form-input"])
            .label("Round")
            .group(&blocker_style)
            .build();

        let chamber_vec: Vec<ChamberObject> =
            vec![ChamberObject::new(None, "-- No Chamber --".to_owned())];
        let model = gio::ListStore::new::<ChamberObject>();
        model.extend_from_slice(&chamber_vec);
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&label));
        });
        factory.connect_bind(clone!(@strong control => move |_, list_item| {
            // Get `ChamberObject` from `ListItem`
            let chamber_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<ChamberObject>()
                .expect("The item has to be an `IntegerObject`.");

            // Get `Label` from `ListItem`
            let label = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<Label>()
                .expect("The child has to be a `Label`.");

            // Set "label" to "chamber name"
            label.set_label(&chamber_object.name().clone());
        }));

        name_i.connect_changed(clone!(@strong control => move |field| {
            let name = field.text().to_string();
            if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_object_id {
                    None => (),
                    Some(object_id) => {
                        control.apply(StateCommand::ChangeObjectName(object_id, name))
                    }
                }
            }
        }));

        notes_i
            .buffer()
            .connect_changed(clone!(@strong control => move |buffer| {
                let (start, end) = buffer.bounds();
                let notes = buffer.text(&start, &end, true).to_string();
                if let Ok(mut control) = control.try_borrow_mut() {
                    match control.state.active_object_id {
                        None => (),
                        Some(object_id) => {
                            if let Some(object) = control.state.dungeon.object(object_id) {
                                if object.notes != notes {
                                    control.apply(StateCommand::ChangeObjectNotes(object_id, notes))
                                };
                            }
                        }
                    }
                }
            }));

        hidden_i.connect_toggled(
            clone!(@strong control => move |w| if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_object_id {
                    None => (),
                    Some(object_id) => control.apply(StateCommand::ChangeObjectHidden(object_id, w.is_active())),
                }
            }),
        );

        blocker_style.connect_toggled(
            clone!(@strong control => move |_w| if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_object_id {
                    None => (),
                    Some(object_id) => control.apply(StateCommand::ChangeObjectStyle(object_id, ObjectStyle::Blocker)),
                }
            }),
        );

        stair_style.connect_toggled(
            clone!(@strong control => move |_w| if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_object_id {
                    None => (),
                    Some(object_id) => control.apply(StateCommand::ChangeObjectStyle(object_id, ObjectStyle::Stairs)),
                }
            }),
        );

        round_style.connect_toggled(
            clone!(@strong control => move |_w| if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_object_id {
                    None => (),
                    Some(object_id) => control.apply(StateCommand::ChangeObjectStyle(object_id, ObjectStyle::Round)),
                }
            }),
        );

        let b = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let part_of_label = Label::builder()
            .label("Part of:")
            .css_classes(vec!["form-input"])
            .build();
        b.append(&part_of_label);
        b.append(&Label::new(Some("Name")));
        b.append(&name_i);
        b.append(&hidden_i);

        b.append(&Label::new(Some("Style")));
        b.append(&blocker_style);
        b.append(&stair_style);
        b.append(&round_style);

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

        b.set_visible(false);

        let re = Rc::new(RefCell::new(ObjectEdit {
            widget: b,
            part_of_label: part_of_label,
            name_input: name_i,
            notes_input: notes_i,
            hidden_input: hidden_i,
            blocker_style: blocker_style,
            stair_style: stair_style,
            round_style: round_style,
        }));

        control.borrow_mut().subscribe_any(re.clone());

        re
    }

    fn show_object(&mut self, state: &crate::state::State) {
        if let Some(object) = state.active_object() {
            match object.part_of {
                Some(chamber_id) => {
                    let chamber = state.dungeon.chamber(chamber_id).unwrap();
                    self.part_of_label
                        .set_text(&format!("Part of: {}", chamber.name));
                }
                None => {
                    self.part_of_label.set_text(&format!("Part of: -"));
                }
            }

            self.name_input.set_text(&object.name);
            self.notes_input.buffer().set_text(&object.notes);
            self.hidden_input.set_active(object.hidden);
            match object.style {
                ObjectStyle::Blocker => self.blocker_style.set_active(true),
                ObjectStyle::Stairs => self.stair_style.set_active(true),
                ObjectStyle::Round => self.round_style.set_active(true),
            };
            self.widget.set_visible(true);
        } else {
            self.widget.set_visible(false);
        }
    }
}

impl StateEventSubscriber for ObjectEdit {
    fn on_state_event(&mut self, state: &crate::state::State, event: StateEvent) {
        match event {
            StateEvent::ActiveObjectChanged(None) => self.show_object(state),
            StateEvent::ActiveObjectChanged(Some(_)) => self.show_object(state),
            StateEvent::Reset => self.widget.set_visible(false),
            StateEvent::Reload => self.show_object(state),
            _ => (),
        }
    }
}
