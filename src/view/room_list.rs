use std::{cell::RefCell, rc::Rc};

use gtk::{Label, ListBox, PolicyType, ScrolledWindow};

use crate::state::{
    commands::menu::SelectRoomCommand, events::StateEvent, State, StateCommand, StateController,
    StateSubscriber,
};

pub struct RoomList {
    pub list_box: ListBox,
    pub scrolled_window: ScrolledWindow,
    pub rows: Vec<Label>,
}
use gtk::prelude::*;

impl RoomList {
    pub fn new(state: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let list_box = ListBox::new();
        let room_list = Rc::new(RefCell::new(RoomList {
            list_box: list_box.clone(),
            scrolled_window: ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
                .min_content_width(360)
                .height_request(600)
                .width_request(180)
                .child(&list_box)
                .build(),
            rows: vec![],
        }));

        {
            let state = state.clone();
            list_box.connect_row_selected(move |_, row| match row {
                Some(row) => unsafe {
                    let room_id = *row
                        .child()
                        .unwrap()
                        .data::<usize>("room_id")
                        .unwrap()
                        .as_ptr();
                    state
                        .borrow_mut()
                        .apply(Box::new(SelectRoomCommand { room_id: room_id }));
                },
                None => {
                    println!("nothing")
                }
            });
        }

        state
            .borrow_mut()
            .subscribe(StateEvent::RoomAdded(0), room_list.clone());
        room_list
    }
}

impl StateSubscriber for RoomList {
    fn on_state_event(
        &mut self,
        state: &mut State,
        _event: StateEvent,
    ) -> Vec<Box<dyn StateCommand>> {
        // remove all
        for row in self.rows.iter() {
            self.list_box.remove(&row.parent().unwrap());
        }
        self.rows.clear();

        for room in state.dungeon.rooms.iter() {
            let room_label = Label::new(Some(
                format!("{} {}", &room.name, room.id.unwrap()).as_str(),
            ));
            unsafe {
                room_label.set_data("room_id", room.id.unwrap());
            }
            self.rows.push(room_label);
            self.list_box.append(self.rows.last().unwrap());
        }

        vec![]
    }
}
