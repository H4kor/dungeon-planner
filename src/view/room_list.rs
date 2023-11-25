use crate::state::{events::StateEvent, State, StateCommand, StateController, StateSubscriber};
use crate::view::room_list_entry::RoomListEntry;
use gtk::prelude::*;
use gtk::{ListBox, PolicyType, ScrolledWindow};
use std::{cell::RefCell, rc::Rc};

pub struct RoomList {
    pub list_box: ListBox,
    pub scrolled_window: ScrolledWindow,
    pub rows: Vec<RoomListEntry>,
}

impl RoomList {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let list_box = ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .build();
        let room_list = Rc::new(RefCell::new(RoomList {
            list_box: list_box.clone(),
            scrolled_window: ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
                .min_content_width(360)
                .height_request(300)
                .width_request(180)
                .child(&list_box)
                .build(),
            rows: vec![],
        }));

        {
            let control = control.clone();
            list_box.connect_row_activated(move |_, row| {
                let room_id = row
                    .clone()
                    .dynamic_cast::<RoomListEntry>()
                    .unwrap()
                    .room_id();
                control
                    .borrow_mut()
                    .apply(StateCommand::SelectRoom(Some(room_id)));
            });
        }

        let mut state = control.borrow_mut();
        state.subscribe(StateEvent::RoomAdded(0), room_list.clone());
        state.subscribe(StateEvent::RoomModified(0), room_list.clone());
        state.subscribe(StateEvent::ActiveRoomChanged(None), room_list.clone());
        room_list
    }
}

impl StateSubscriber for RoomList {
    fn on_state_event(&mut self, state: &mut State, event: StateEvent) -> Vec<StateCommand> {
        match event {
            StateEvent::RoomAdded(room_id) => {
                let room = state.dungeon.room(room_id).unwrap();
                let room_label = RoomListEntry::new(room);
                self.rows.push(room_label);
                self.list_box.append(self.rows.last().unwrap());
            }
            StateEvent::RoomModified(room_id) => {
                let room = state.dungeon.room(room_id).unwrap();
                self.rows
                    .iter_mut()
                    .filter(|r| r.room_id() == room_id)
                    .for_each(|w| w.update(room));
            }
            StateEvent::ActiveRoomChanged(room_id) => match room_id {
                None => self.list_box.unselect_all(),
                Some(room_id) => self
                    .list_box
                    .select_row(self.rows.iter().find(|r| r.room_id() == room_id)),
            },
        }
        vec![]
    }
}
