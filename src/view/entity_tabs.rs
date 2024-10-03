use std::{cell::RefCell, rc::Rc};

use gtk::{Label, Notebook};

use crate::state::{events::StateEvent, State, StateController, StateEventSubscriber};

pub struct EntityTabs {
    pub widget: Notebook,
}

impl EntityTabs {
    pub fn new(
        control: Rc<RefCell<StateController>>,
        dungeon_tab: gtk::Box,
        chamber_tab: gtk::Box,
        door_tab: gtk::Box,
        object_tab: gtk::Box,
    ) -> Rc<RefCell<Self>> {
        let notebook = Notebook::builder().build();

        notebook.append_page(&dungeon_tab, Some(&Label::new(Some("Dungeon"))));
        notebook.append_page(&chamber_tab, Some(&Label::new(Some("Chambers"))));
        notebook.append_page(&door_tab, Some(&Label::new(Some("Doors"))));
        notebook.append_page(&object_tab, Some(&Label::new(Some("Objects"))));

        let tabs = Rc::new(RefCell::new(EntityTabs { widget: notebook }));
        control.borrow_mut().subscribe_any(tabs.clone());

        tabs
    }
}

impl StateEventSubscriber for EntityTabs {
    fn on_state_event(&mut self, _state: &State, event: StateEvent) {
        match event {
            StateEvent::ActiveChamberChanged(Some(_)) => {
                self.widget.set_current_page(Some(1));
            }
            StateEvent::ActiveDoorChanged(Some(_)) => {
                self.widget.set_current_page(Some(2));
            }
            StateEvent::ActiveObjectChanged(Some(_)) => {
                self.widget.set_current_page(Some(3));
            }
            StateEvent::DoorAdded(_) => {
                self.widget.set_current_page(Some(2));
            }
            StateEvent::ObjectAdded(_) => {
                self.widget.set_current_page(Some(3));
            }
            _ => {}
        };
    }
}
