use std::{cell::RefCell, rc::Rc};

use crate::state::EditMode;
use crate::state::StateCommand;
use crate::state::StateController;
use gtk::prelude::*;
use gtk::Button;

pub struct AddRoomButton {
    pub widget: Button,
}
pub struct SelectRoomButton {
    pub widget: Button,
}

pub struct SplitEdgeButton {
    pub widget: Button,
}
pub struct AppendRoomButton {
    pub widget: Button,
}

impl AddRoomButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder().icon_name("document-new").build();

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(StateCommand::AddRoom);
        });

        AddRoomButton { widget: button }
    }
}

impl SelectRoomButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder().icon_name("edit-find").build();

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(StateCommand::ChangeMode(EditMode::Select));
        });

        SelectRoomButton { widget: button }
    }
}

impl SplitEdgeButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder().icon_name("edit-cut").build();

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(StateCommand::ChangeMode(EditMode::SplitEdge));
        });

        SplitEdgeButton { widget: button }
    }
}

impl AppendRoomButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder().icon_name("document-edit").build();

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(StateCommand::ChangeMode(EditMode::AppendRoom));
        });

        AppendRoomButton { widget: button }
    }
}
