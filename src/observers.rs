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
    save_file: String,
    mode: PersistenceMode,
    cmds: Vec<StateCommand>,
}

impl HistoryObserver {
    pub fn new(state: Rc<RefCell<StateController>>, save_file: String) -> Rc<RefCell<Self>> {
        let obs = Rc::new(RefCell::new(HistoryObserver {
            save_file: save_file,
            mode: PersistenceMode::Restoring,
            cmds: vec![],
        }));

        state.borrow_mut().subscribe_cmds(obs.clone());
        obs
    }

    pub fn change_file(&mut self, new_file: String) {
        self.save_file = new_file;
        self.mode = PersistenceMode::Restoring;
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
}

impl StateCommandSubscriber for HistoryObserver {
    fn on_cmd_event(&mut self, _state: &mut crate::state::State, cmd: StateCommand) {
        if self.mode == PersistenceMode::Record || self.mode == PersistenceMode::Restoring {
            self.cmds.push(cmd);
            storage::save_to_file(self.save_file.clone(), &self.cmds);
        }
    }
}
