use std::{cell::RefCell, rc::Rc};

use crate::room::Room;
use crate::state::commands::AddRoomCommand;
use crate::state::StateController;
use gtk::prelude::*;
use gtk::Button;

pub struct AddRoomButton {
    pub widget: Button,
}

impl AddRoomButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder().label("New Room").build();

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(Box::new(AddRoomCommand::new(Room::new(None))));
        });

        AddRoomButton { widget: button }
    }
}
