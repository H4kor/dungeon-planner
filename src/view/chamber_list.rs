use crate::state::{
    events::StateEvent, State, StateCommand, StateController, StateEventSubscriber,
};
use crate::view::chamber_list_entry::ChamberListEntry;
use cairo::glib::{clone, Propagation};
use gtk::{gdk, prelude::*, EventControllerKey};
use gtk::{ListBox, PolicyType, ScrolledWindow};
use std::{cell::RefCell, rc::Rc};

pub struct ChamberList {
    pub list_box: ListBox,
    pub scrolled_window: ScrolledWindow,
    pub rows: Vec<ChamberListEntry>,
}

impl ChamberList {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let list_box = ListBox::builder()
            .selection_mode(gtk::SelectionMode::Single)
            .focusable(true)
            .focus_on_click(true)
            .build();
        let chamber_list = Rc::new(RefCell::new(ChamberList {
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
            let chamber_id = row
                .clone()
                .dynamic_cast::<ChamberListEntry>()
                .unwrap()
                .chamber_id();
            control
                .borrow_mut()
                .apply(StateCommand::SelectChamber(Some(chamber_id)));
        }));

        let key_controller = EventControllerKey::new();
        key_controller.connect_key_pressed(
            clone!(@strong control, @strong list_box => move |_, key, _, _| {
                match key {
                    gdk::Key::Delete => {
                        if let Some(row) = list_box.selected_row() {
                            let chamber_id = row
                                .clone()
                                .dynamic_cast::<ChamberListEntry>()
                                .unwrap()
                                .chamber_id();
                            control
                                .borrow_mut()
                                .apply(StateCommand::DeleteChamber(chamber_id))
                        }
                    },
                    _ => (),
                }
                Propagation::Proceed
            }),
        );
        list_box.add_controller(key_controller);

        let mut state = control.borrow_mut();
        state.subscribe_any(chamber_list.clone());
        chamber_list
    }

    fn rebuild_list(&mut self, state: &State) {
        for row in &self.rows {
            self.list_box.remove(row)
        }
        self.rows = vec![];
        for chamber in &state.dungeon.chambers {
            let chamber_label = ChamberListEntry::new(&chamber.clone());
            self.rows.push(chamber_label);
            self.list_box.append(self.rows.last().unwrap());
        }
        match state.active_chamber_id {
            None => self.list_box.unselect_all(),
            Some(chamber_id) => self
                .list_box
                .select_row(self.rows.iter().find(|r| r.chamber_id() == chamber_id)),
        }
    }
}

impl StateEventSubscriber for ChamberList {
    fn on_state_event(&mut self, state: &mut State, event: StateEvent) -> Vec<StateCommand> {
        match event {
            StateEvent::ChamberAdded(_) => {
                self.rebuild_list(state);
            }
            StateEvent::ChamberModified(chamber_id) => {
                let chamber = state.dungeon.chamber_mut(chamber_id).unwrap();
                self.rows
                    .iter_mut()
                    .filter(|r| r.chamber_id() == chamber_id)
                    .for_each(|w| w.update(chamber));
            }
            StateEvent::ActiveChamberChanged(chamber_id) => match chamber_id {
                None => self.list_box.unselect_all(),
                Some(chamber_id) => self
                    .list_box
                    .select_row(self.rows.iter().find(|r| r.chamber_id() == chamber_id)),
            },
            StateEvent::Reset => self.rebuild_list(state),
            StateEvent::ChamberDeleted(_) => self.rebuild_list(state),
            _ => (),
        }
        vec![]
    }
}
