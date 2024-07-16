use std::{cell::RefCell, rc::Rc};

use crate::state::events::StateEvent;
use crate::state::EditMode;
use crate::state::StateCommand;
use crate::state::StateController;
use crate::state::StateEventSubscriber;
use cairo::glib::clone;
use gtk::gdk::Texture;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::Cancellable;
use gtk::gio::MemoryInputStream;
use gtk::glib;
use gtk::prelude::*;
use gtk::Image;
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
        let button = Button::new();
        let bytes = include_bytes!("../../assets/icons/add_chamber.png");
        let bytes = glib::Bytes::from(&bytes.to_vec());
        let stream = MemoryInputStream::from_bytes(&bytes);
        let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
        let texture = Texture::for_pixbuf(&pixbuf);
        let image = Image::from_paintable(Some(&texture));
        button.set_child(Some(&image));
        button.set_tooltip_text(Some("Create new Chamber"));
        button.set_has_tooltip(true);
        button.set_size_request(64, 64);

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
