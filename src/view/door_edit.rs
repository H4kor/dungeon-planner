use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{gio, glib, DropDown, Expression, ListItem, SignalListItemFactory};
use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::state::events::StateEvent;
use crate::state::{StateCommand, StateController, StateEventSubscriber};

use super::room_list_object::RoomObject;

pub struct DoorEdit {
    pub widget: Box,
    name_input: Entry,
    notes_input: TextView,
    leads_to_input: DropDown,
    rooms_model: gio::ListStore,
}

impl DoorEdit {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let name_i = Entry::builder().build();
        let notes_i = TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .left_margin(10)
            .right_margin(10)
            .build();

        let mut room_vec: Vec<RoomObject> = vec![RoomObject::new(None, "-- No Room --".to_owned())];
        // room_vec.append(
        //     control
        //         .borrow()
        //         .dungeon()
        //         .rooms
        //         .iter()
        //         .map(|r| RoomObject::new(r.id, r.name.clone()))
        //         .collect(),
        // );
        let model = gio::ListStore::new::<RoomObject>();
        model.extend_from_slice(&room_vec);
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            println!("SETUP");
            let label = Label::new(None);
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&label));
        });
        factory.connect_bind(clone!(@strong control => move |_, list_item| {
            println!("BINDING");
            // Get `RoomObject` from `ListItem`
            let room_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<RoomObject>()
                .expect("The item has to be an `IntegerObject`.");

            // Get `Label` from `ListItem`
            let label = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<Label>()
                .expect("The child has to be a `Label`.");

            // Set "label" to "room name"
            label.set_label(&room_object.name().clone());
        }));

        let leads_to_i = DropDown::builder().build();
        leads_to_i.set_factory(Some(&factory));
        leads_to_i.set_model(Some(&model));
        leads_to_i.set_expression(Expression::NONE);

        name_i.connect_changed(clone!(@strong control => move |field| {
            let name = field.text().to_string();
            if let Ok(mut control) = control.try_borrow_mut() {
                match control.state.active_room_id {
                    None => (),
                    Some(room_id) => control.apply(StateCommand::ChangeRoomName(room_id, name)),
                }
            }
        }));

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
        b.append(&Label::new(Some("Leads to Room:")));
        b.append(&leads_to_i);

        b.set_visible(false);

        let re = Rc::new(RefCell::new(DoorEdit {
            widget: b,
            name_input: name_i,
            notes_input: notes_i,
            leads_to_input: leads_to_i,
            rooms_model: model,
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
            StateEvent::RoomAdded(room_id) => self.rooms_model.append(&RoomObject::new(
                Some(room_id),
                state.dungeon.room(room_id).unwrap().name.clone(),
            )),
            _ => (),
        }
        vec![]
    }
}
