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
        _state: &mut crate::state::State,
        event: crate::state::events::StateEvent,
    ) -> Vec<StateCommand> {
        println!("{:#?}", event);
        vec![]
    }
}

#[derive(PartialEq)]
enum PersistenceMode {
    Restoring, // initial state <- data being restored from file
    Record,    // normal mode <- recording all commands
    Replaying, // during undo <- commands are not added to vector
}

pub struct HistoryObserver {
    save_file: Option<String>,
    mode: PersistenceMode,
    cmds: Vec<StateCommand>,
}

impl HistoryObserver {
    pub fn new(
        state: Rc<RefCell<StateController>>,
        save_file: Option<String>,
    ) -> Rc<RefCell<Self>> {
        let obs = Rc::new(RefCell::new(HistoryObserver {
            save_file: save_file,
            mode: PersistenceMode::Restoring,
            cmds: vec![],
        }));

        state.borrow_mut().subscribe_cmds(obs.clone());
        obs
    }

    pub fn change_file(&mut self, new_file: String) {
        self.save_file = Some(new_file);
    }
    pub fn reset(&mut self) {
        self.save_file = None;
        self.cmds = vec![];
    }

    pub fn activate(&mut self) {
        self.mode = PersistenceMode::Record
    }

    pub fn undo(&mut self) {
        self.cmds.pop();
    }

    pub fn get_stack(&self) -> Vec<StateCommand> {
        self.cmds.clone()
    }
    pub fn end_restore(&mut self) {
        self.mode = PersistenceMode::Record
    }
    pub fn start_restore(&mut self) {
        self.mode = PersistenceMode::Replaying
    }

    pub fn save_file(&self) -> Option<String> {
        self.save_file.clone()
    }

    pub fn save_to_file(&self) {
        match &self.save_file {
            Some(f) => storage::save_to_file(f.to_string(), &self.cmds),
            None => todo!(),
        }
    }
}

impl StateCommandSubscriber for HistoryObserver {
    fn on_cmd_event(&mut self, _state: &mut crate::state::State, cmd: StateCommand) {
        if self.mode == PersistenceMode::Record || self.mode == PersistenceMode::Restoring {
            self.cmds.push(cmd);
        }
    }
}
