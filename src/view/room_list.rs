use std::{cell::RefCell, rc::Rc};

use gtk::{Label, ListBox, PolicyType, ScrolledWindow};

use crate::state::{events::StateEvent, State, StateCommand, StateController, StateSubscriber};

pub struct RoomList {
    pub list_box: ListBox,
    pub scrolled_window: ScrolledWindow,
    pub rows: Vec<Label>,
}

impl RoomList {
    pub fn new(state: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let list_box = ListBox::new();
        let room_list = Rc::new(RefCell::new(RoomList {
            list_box: list_box.clone(),
            scrolled_window: ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
                .min_content_width(360)
                .width_request(180)
                .child(&list_box)
                .build(),
            rows: vec![],
        }));
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
        event: StateEvent,
    ) -> Vec<Box<dyn StateCommand>> {
        for row in self.rows.iter() {
            self.list_box.remove(row);
        }

        for room in state.dungeon.rooms.iter() {
            let room_label = Label::new(Some(&room.name));
            self.rows.push(room_label);
            self.list_box.append(self.rows.last().unwrap());
        }

        vec![]
    }
}
