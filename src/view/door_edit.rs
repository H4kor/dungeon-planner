use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{gio, glib, CheckButton, DropDown, Expression, ListItem, SignalListItemFactory};
use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::chamber::ChamberId;
use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

use super::chamber_list_object::ChamberObject;

pub struct DoorEdit {
    pub widget: Box,
    part_of_label: Label,
    name_input: Entry,
    notes_input: TextView,
    leads_to_input: DropDown,
    hidden_input: CheckButton,
    chambers_model: gio::ListStore,
}

impl DoorEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().build();
        let notes_i = TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .left_margin(10)
            .right_margin(10)
            .build();
        let hidden_i = CheckButton::builder().label("Hidden").build();

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

        let leads_to_i = DropDown::builder().build();
        leads_to_i.set_factory(Some(&factory));
        leads_to_i.set_model(Some(&model));
        leads_to_i.set_expression(Expression::NONE);

        leads_to_i.connect_selected_item_notify(clone!(@weak control => move |drop_down| {
            let model = drop_down.model().expect("The model has to exist.");
            let chamber_object = model
                .item(drop_down.selected())
                .and_downcast::<ChamberObject>()
                .expect("The item has to be an `ChamberObject`.");

            if let Ok(mut control) = control.try_borrow_mut() {
                let door_id = control.state.active_door_id.unwrap();
                control.apply(StateCommand::ChangeDoorLeadsTo(door_id, match chamber_object.valid() {
                    true => Some(chamber_object.chamber()),
                    false => None,
                }));
            };
        }));

        name_i.connect_changed(clone!(@strong control => move |field| {
            let name = field.text().to_string();
            if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_door_id {
                    None => (),
                    Some(door_id) => {
                        control.apply(StateCommand::ChangeDoorName(door_id, name))
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
                }
            }));

        hidden_i.connect_toggled(
            clone!(@strong control => move |w| if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_door_id {
                    None => (),
                    Some(door_id) => control.apply(StateCommand::ChangeDoorHidden(door_id, w.is_active())),
                }
            }),
        );

        let b = Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .build();

        let part_of_label = Label::new(Some("Part of:"));
        b.append(&part_of_label);
        b.append(&Label::new(Some("Name")));
        b.append(&name_i);
        b.append(&hidden_i);
        b.append(&Label::new(Some("Notes")));
        b.append(&notes_i);
        b.append(&Label::new(Some("Leads to Chamber:")));
        b.append(&leads_to_i);

        b.set_visible(false);

        let re = Rc::new(RefCell::new(DoorEdit {
            widget: b,
            part_of_label: part_of_label,
            name_input: name_i,
            notes_input: notes_i,
            leads_to_input: leads_to_i,
            hidden_input: hidden_i,
            chambers_model: model,
        }));

        control.borrow_mut().subscribe_any(re.clone());

        re
    }

    fn chamber_object_pos(&self, id: Option<ChamberId>) -> u32 {
        self.chambers_model
            .into_iter()
            .position(|r| {
                Some(
                    r.expect("Expected Chamber Object")
                        .downcast::<ChamberObject>()
                        .expect("The item has to be an `IntegerObject`.")
                        .chamber(),
                ) == id
            })
            .unwrap_or(0) as u32
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
                let door = state.dungeon.door(door_id).unwrap();
                let chamber = state.dungeon.chamber(door.part_of).unwrap();
                self.part_of_label
                    .set_text(&format!("Part of: {}", chamber.name));

                self.name_input.set_text(&door.name);
                self.notes_input.buffer().set_text(&door.notes);
                self.leads_to_input
                    .set_selected(self.chamber_object_pos(door.leads_to));
                self.hidden_input.set_active(door.hidden);
                self.widget.set_visible(true);
            }
            StateEvent::Reset => self.widget.set_visible(false),
            StateEvent::ChamberAdded(chamber_id) => {
                self.chambers_model.append(&ChamberObject::new(
                    Some(chamber_id),
                    state.dungeon.chamber(chamber_id).unwrap().name.clone(),
                ))
            }
            StateEvent::ChamberModified(chamber_id) => {
                let chamber_object = self
                    .chambers_model
                    .item(self.chamber_object_pos(Some(chamber_id)))
                    .unwrap()
                    .downcast::<ChamberObject>()
                    .expect("The item has to be an `IntegerObject`.");
                chamber_object.set_name(state.dungeon.chamber(chamber_id).unwrap().name.clone())
            }
            _ => (),
        }
        vec![]
    }
}
