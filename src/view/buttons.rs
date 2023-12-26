use std::{cell::RefCell, rc::Rc};

use crate::state::events::StateEvent;
use crate::state::EditMode;
use crate::state::StateCommand;
use crate::state::StateController;
use crate::state::StateEventSubscriber;
use cairo::glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::{Button, ToggleButton};
pub struct AddChamberButton {
    pub widget: Button,
}
pub struct EditModeButton {
    pub widget: ToggleButton,
    mode: EditMode,
}

impl AddChamberButton {
    pub fn new(control: Rc<RefCell<StateController>>) -> Self {
        let button = Button::builder()
            .icon_name("document-new")
            .tooltip_text("Create new Chamber")
            .has_tooltip(true)
            .build();
        button.set_size_request(48, 48);

        button.connect_clicked(move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(StateCommand::AddChamber);
        });

        AddChamberButton { widget: button }
    }
}

impl EditModeButton {
    pub fn new(
        control: Rc<RefCell<StateController>>,
        mode: EditMode,
        icon_name: &str,
        tooltip: &str,
    ) -> Rc<RefCell<Self>> {
        let button = ToggleButton::builder()
            .icon_name(icon_name)
            .tooltip_text(tooltip)
            .has_tooltip(true)
            .build();
        button.set_size_request(48, 48);

        button.connect_clicked(clone!( @weak control => move |_button| {
            let control = &mut *control.borrow_mut();
            control.apply(StateCommand::ChangeMode(mode));
        }));

        let b = Rc::new(RefCell::new(EditModeButton {
            widget: button,
            mode: mode,
        }));

        control
            .borrow_mut()
            .subscribe(StateEvent::EditModeChanged(mode), b.clone());
        control
            .borrow_mut()
            .subscribe(StateEvent::Reload, b.clone());
        b
    }
}

impl StateEventSubscriber for EditModeButton {
    fn on_state_event(
        &mut self,
        state: &crate::state::State,
        event: StateEvent,
    ) -> Vec<StateCommand> {
        match event {
            StateEvent::EditModeChanged(mode) => self.widget.set_active(mode == self.mode),
            StateEvent::Reload => self.widget.set_active(state.mode == self.mode),
            _ => (),
        }
        vec![]
    }
}
