use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib::clone;
use gtk::{gio, glib, DropDown, Expression, ListItem, SignalListItemFactory};
use gtk::{prelude::*, Label, TextView};
use gtk::{Box, Entry};

use crate::room::RoomId;
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

        let room_vec: Vec<RoomObject> = vec![RoomObject::new(None, "-- No Room --".to_owned())];
        let model = gio::ListStore::new::<RoomObject>();
        model.extend_from_slice(&room_vec);
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&label));
        });
        factory.connect_bind(clone!(@strong control => move |_, list_item| {
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

        leads_to_i.connect_selected_item_notify(clone!(@weak control => move |drop_down| {
            let model = drop_down.model().expect("The model has to exist.");
            let room_object = model
                .item(drop_down.selected())
                .and_downcast::<RoomObject>()
                .expect("The item has to be an `RoomObject`.");

            if let Ok(mut control) = control.try_borrow_mut() {
                let door_id = control.state.active_door_id.unwrap();
                control.apply(StateCommand::ChangeDoorLeadsTo(door_id, match room_object.valid() {
                    true => Some(room_object.room()),
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

    fn room_object_pos(&self, id: Option<RoomId>) -> u32 {
        self.rooms_model
            .into_iter()
            .position(|r| {
                Some(
                    r.expect("Expected Room Object")
                        .downcast::<RoomObject>()
                        .expect("The item has to be an `IntegerObject`.")
                        .room(),
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
                let door = state.dungeon.door_mut(door_id).unwrap();
                self.name_input.set_text(&door.name);
                self.notes_input.buffer().set_text(&door.notes);
                self.leads_to_input
                    .set_selected(self.room_object_pos(door.leads_to));
                self.widget.set_visible(true);
            }
            StateEvent::Reset => self.widget.set_visible(false),
            StateEvent::RoomAdded(room_id) => self.rooms_model.append(&RoomObject::new(
                Some(room_id),
                state.dungeon.room(room_id).unwrap().name.clone(),
            )),
            StateEvent::RoomModified(room_id) => {
                let room_object = self
                    .rooms_model
                    .item(self.room_object_pos(Some(room_id)))
                    .unwrap()
                    .downcast::<RoomObject>()
                    .expect("The item has to be an `IntegerObject`.");
                room_object.set_name(state.dungeon.room(room_id).unwrap().name.clone())
            }
            _ => (),
        }
        vec![]
    }
}
