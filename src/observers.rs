use crate::state::{
    State, StateCommand, StateCommandSubscriber, StateController, StateEventSubscriber,
};
use crate::storage;
use std::{cell::RefCell, rc::Rc};
pub struct DebugObserver {}

const MAX_EDIT_DISTANCE: i64 = 12;

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
        for cmd in cmds {
            self.on_cmd_event(&mut State::new(), cmd)
        }
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
        match cmd {
            // compressing text changes into reasonable chunks
            StateCommand::ChangeChamberNotes(id, s) => {
                crate::txt_cmd!(StateCommand::ChangeChamberNotes, id, s, self.cmds);
            }
            StateCommand::ChangeChamberName(id, s) => {
                crate::txt_cmd!(StateCommand::ChangeChamberName, id, s, self.cmds);
            }
            StateCommand::ChangeDoorNotes(id, s) => {
                crate::txt_cmd!(StateCommand::ChangeDoorNotes, id, s, self.cmds);
            }
            StateCommand::ChangeDoorName(id, s) => {
                crate::txt_cmd!(StateCommand::ChangeDoorName, id, s, self.cmds);
            }
            StateCommand::ChangeDungeonName(s) => {
                crate::txt_cmd_dungeon!(StateCommand::ChangeDungeonName, s, self.cmds);
            }
            StateCommand::ChangeDungeonNotes(s) => {
                crate::txt_cmd_dungeon!(StateCommand::ChangeDungeonNotes, s, self.cmds);
            }
            x => {
                self.cmds.push(x);
            }
        }
        self.unsaved_state = true;
    }
}

#[macro_export]
macro_rules! txt_cmd {
    ( $x:path, $i:expr, $s:expr, $c:expr ) => {{
        let mut append = true;

        let cmd_len = $c.len();
        if cmd_len > 2 {
            let prev_cmd = $c[cmd_len - 1].clone();
            let prev_prev_cmd = $c[cmd_len - 2].clone();

            // check if prev and prev_prev are same type as current
            if let ($x(prev_id, _), $x(prev_prev_id, prev_prev_s)) = (prev_cmd, prev_prev_cmd) {
                if prev_id == prev_prev_id && prev_id == $i {
                    // simplified distance to keep computation cheap
                    let dist = ($s.len() as i64 - prev_prev_s.len() as i64).abs();
                    if dist < MAX_EDIT_DISTANCE {
                        let p = $c.len() - 1;
                        $c[p] = $x($i, $s.clone());
                        append = false;
                    }
                }
            }
        }
        if append {
            $c.push($x($i, $s))
        }
    }};
}

#[macro_export]
macro_rules! txt_cmd_dungeon {
    ( $x:path, $s:expr, $c:expr ) => {{
        let mut append = true;

        let cmd_len = $c.len();
        if cmd_len > 2 {
            let prev_cmd = $c[cmd_len - 1].clone();
            let prev_prev_cmd = $c[cmd_len - 2].clone();

            // check if prev and prev_prev are same type as current
            if let ($x(_), $x(prev_prev_s)) = (prev_cmd, prev_prev_cmd) {
                // simplified distance to keep computation cheap
                let dist = ($s.len() as i64 - prev_prev_s.len() as i64).abs();
                if dist < MAX_EDIT_DISTANCE {
                    let p = $c.len() - 1;
                    $c[p] = $x($s.clone());
                    append = false;
                }
            }
        }
        if append {
            $c.push($x($s))
        }
    }};
}
