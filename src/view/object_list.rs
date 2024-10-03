use crate::state::{
    events::StateEvent, State, StateCommand, StateController, StateEventSubscriber,
};
use crate::view::object_list_entry::ObjectListEntry;
use cairo::glib::{clone, Propagation};
use gtk::{gdk, prelude::*, EventControllerKey};
use gtk::{ListBox, PolicyType, ScrolledWindow};
use std::{cell::RefCell, rc::Rc};

pub struct ObjectList {
    pub list_box: ListBox,
    pub scrolled_window: ScrolledWindow,
    pub rows: Vec<ObjectListEntry>,
}

impl ObjectList {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let list_box = ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .build();
        let object_list = Rc::new(RefCell::new(ObjectList {
            list_box: list_box.clone(),
            scrolled_window: ScrolledWindow::builder()
                .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
                .min_content_width(360)
                .height_request(300)
                .child(&list_box)
                .build(),
            rows: vec![],
        }));

        list_box.connect_row_activated(clone!(@strong control => move |_, row| {
            let object_id = row
                .clone()
                .dynamic_cast::<ObjectListEntry>()
                .unwrap()
                .object_id();
            control
                .borrow_mut()
                .apply(StateCommand::SelectObject(Some(object_id)));
        }));

        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(
            clone!(@strong control, @strong list_box => move |_, key, _, _| {
                match key {
                    gdk::Key::Delete => {
                        if let Some(row) = list_box.selected_row() {
                            let object_id = row
                                .clone()
                                .dynamic_cast::<ObjectListEntry>()
                                .unwrap()
                                .object_id();
                            control
                                .borrow_mut()
                                .apply(StateCommand::DeleteObject(object_id))
                        }
                    },
                    _ => (),
                }
                Propagation::Proceed
            }),
        );
        list_box.add_controller(key_controller);

        let mut state = control.borrow_mut();
        state.subscribe_any(object_list.clone());
        object_list
    }

    fn rebuild_list(&mut self, state: &State) {
        for row in &self.rows {
            self.list_box.remove(row)
        }
        self.rows = vec![];
        for object in &state.dungeon.objects {
            let object_label = ObjectListEntry::new(&object.clone());
            self.rows.push(object_label);
            self.list_box.append(self.rows.last().unwrap());
        }
        match state.active_object_id {
            None => self.list_box.unselect_all(),
            Some(object_id) => self
                .list_box
                .select_row(self.rows.iter().find(|r| r.object_id() == object_id)),
        }
    }
}

impl StateEventSubscriber for ObjectList {
    fn on_state_event(&mut self, state: &State, event: StateEvent) {
        match event {
            StateEvent::ObjectAdded(_) => {
                self.rebuild_list(state);
            }
            StateEvent::ObjectModified(object_id) => {
                let object = state.dungeon.object(object_id).unwrap();
                self.rows
                    .iter_mut()
                    .filter(|r| r.object_id() == object_id)
                    .for_each(|w| w.update(object));
            }
            StateEvent::ActiveObjectChanged(object_id) => match object_id {
                None => self.list_box.unselect_all(),
                Some(object_id) => self
                    .list_box
                    .select_row(self.rows.iter().find(|r| r.object_id() == object_id)),
            },
            StateEvent::Reset => self.rebuild_list(state),
            StateEvent::Reload => self.rebuild_list(state),
            StateEvent::ObjectDeleted(_) => self.rebuild_list(state),
            _ => (),
        }
    }
}
