use std::{
    cell::RefCell,
    fs::{File, OpenOptions},
    rc::Rc,
};

use crate::state::{
    commands::AddRoomCommand, StateCommand, StateCommandSubscriber, StateController,
    StateSubscriber,
};
use std::io::prelude::*;

pub struct DebugObserver {}

impl DebugObserver {
    pub fn new(state: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let obs = Rc::new(RefCell::new(DebugObserver {}));

        state.borrow_mut().subscribe_any(obs.clone());
        obs
    }
}

impl StateSubscriber for DebugObserver {
    fn on_state_event(
        &mut self,
        _state: &mut crate::state::State,
        event: crate::state::events::StateEvent,
    ) -> Vec<RefCell<std::boxed::Box<dyn StateCommand>>> {
        println!("{:#?}", event);
        vec![]
    }
}

pub struct StorageObserver {
    file: File,
}

impl StorageObserver {
    pub fn new(state: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        // let file = File::create("dungeon.txt").unwrap();
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("dungeon.txt")
            .unwrap();
        let obs = Rc::new(RefCell::new(StorageObserver { file: file }));

        state.borrow_mut().subscribe_cmds(obs.clone());
        obs
    }
}

impl StateCommandSubscriber for StorageObserver {
    fn on_cmd_event(&mut self, _state: &mut crate::state::State, cmd: &dyn StateCommand) {
        writeln!(self.file, "{} >> {}", cmd.data().name, cmd.data().data).unwrap();
    }
}
