use std::{cell::RefCell, rc::Rc};

use crate::state::commands::AddRoomCommand;
use crate::state::StateController;
use gtk::prelude::*;
use gtk::Button;

pub struct AddRoomButton {
    pub widget: Button,
}

impl AddRoomButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder().icon_name("document-new").build();

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(RefCell::new(Box::new(AddRoomCommand {})));
        });

        AddRoomButton { widget: button }
    }
}
