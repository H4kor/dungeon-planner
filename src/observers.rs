use crate::state::{StateCommand, StateCommandSubscriber, StateController, StateEventSubscriber};
use crate::storage;
use std::{cell::RefCell, rc::Rc};

pub struct DebugObserver {}

impl DebugObserver {
    pub fn new(state: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let obs = Rc::new(RefCell::new(DebugObserver {}));

        state.borrow_mut().subscribe_any(obs.clone());
        obs
    }
}

impl StateEventSubscriber for DebugObserver {
    fn on_state_event(
        &mut self,
        _state: &crate::state::State,
        event: crate::state::events::StateEvent,
    ) -> Vec<StateCommand> {
        println!("{:#?}", event);
        vec![]
    }
}

pub struct HistoryObserver {
    save_file: Option<String>,
    cmds: Vec<StateCommand>,
    unsaved_state: bool,
}

impl HistoryObserver {
    pub fn new(
        state: Rc<RefCell<StateController>>,
        save_file: Option<String>,
    ) -> Rc<RefCell<Self>> {
        let obs = Rc::new(RefCell::new(HistoryObserver {
            save_file: save_file,
            cmds: vec![],
            unsaved_state: false,
        }));

        state.borrow_mut().subscribe_cmds(obs.clone());
        obs
    }

    pub fn change_file(&mut self, new_file: String) {
        self.save_file = Some(new_file);
    }
    pub fn reset(&mut self) {
        self.save_file = None;
        self.unsaved_state = false;
        self.cmds = vec![];
    }

    pub fn undo(&mut self) {
        self.cmds.pop();
    }

    pub fn get_stack(&self) -> Vec<StateCommand> {
        self.cmds.clone()
    }

    pub fn save_file(&self) -> Option<String> {
        self.save_file.clone()
    }

    pub fn unsaved_state(&self) -> bool {
        self.unsaved_state.clone()
    }

    pub fn set_history(&mut self, cmds: Vec<StateCommand>) {
        self.cmds = cmds;
    }

    pub fn save_to_file(&mut self) {
        match &self.save_file {
            Some(f) => {
                storage::save_to_file(f.to_string(), &self.cmds);
                self.unsaved_state = false;
            }
            None => todo!(),
        }
    }
}

impl StateCommandSubscriber for HistoryObserver {
    fn on_cmd_event(&mut self, _state: &mut crate::state::State, cmd: StateCommand) {
        self.cmds.push(cmd);
        self.unsaved_state = true;
    }
}
