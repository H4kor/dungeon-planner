use std::{
    cell::RefCell,
    fs::{File, OpenOptions},
    rc::Rc,
};

use serde_json::json;

use crate::state::{State, StateCommand, StateCommandSubscriber, StateController, StateSubscriber};
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
    ) -> Vec<StateCommand> {
        println!("{:#?}", event);
        vec![]
    }
}

pub struct StorageObserver {
    file: File,
    active: bool,
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
        let obs = Rc::new(RefCell::new(StorageObserver {
            file: file,
            active: false,
        }));

        state.borrow_mut().subscribe_cmds(obs.clone());
        obs
    }
    pub fn deactivate(&mut self) {
        self.active = false
    }
    pub fn activate(&mut self) {
        self.active = true
    }
}

impl StateCommandSubscriber for StorageObserver {
    fn on_cmd_event(&mut self, _state: &mut crate::state::State, cmd: StateCommand) {
        if !self.active {
            return;
        }
        let name = match cmd {
            StateCommand::AddRoom => "AddRoomCommand".to_owned(),
            StateCommand::SelectRoom(_) => "SelectRoomCommand".to_owned(),
            StateCommand::AddVertexToRoom(_, _) => "AddVertexToRoomCommand".to_owned(),
            StateCommand::ChangeRoomName(_, _) => "ChangeRoomName".to_owned(),
            StateCommand::ChangeRoomNodes(_, _) => "ChangeRoomNotes".to_owned(),
        };
        let data = match cmd {
            StateCommand::AddRoom => serde_json::Value::Null,
            StateCommand::SelectRoom(room_id) => json!({"room_id": room_id}),
            StateCommand::AddVertexToRoom(room_id, pos) => json!({
                "room_id": room_id,
                "x": pos.x,
                "y": pos.y
            }),
            StateCommand::ChangeRoomName(room_id, name) => json!({
                "room_id": room_id,
                "name": name,
            }),
            StateCommand::ChangeRoomNodes(room_id, notes) => json!({
                "room_id": room_id,
                "notes": notes,
            }),
        };
        writeln!(self.file, "{} >> {}", name, data).unwrap();
    }
}

pub struct UndoObserver {
    cmds: Vec<StateCommand>,
    active: bool,
}

impl UndoObserver {
    pub fn new(control: Rc<RefCell<StateController>>) -> Rc<RefCell<Self>> {
        let obs = Rc::new(RefCell::new(UndoObserver {
            cmds: vec![],
            active: true,
        }));
        control.borrow_mut().subscribe_cmds(obs.clone());
        obs
    }

    pub fn undo(&mut self) {
        self.cmds.pop();
    }

    pub fn get_stack(&self) -> Vec<StateCommand> {
        self.cmds.clone()
    }
    pub fn end_restore(&mut self) {
        self.active = true
    }
    pub fn start_restore(&mut self) {
        self.active = false
    }
}

impl StateCommandSubscriber for UndoObserver {
    fn on_cmd_event(&mut self, state: &mut State, cmd: StateCommand) {
        if self.active {
            self.cmds.push(cmd)
        }
    }
}
