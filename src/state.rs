mod commands;
pub mod events;

use crate::{
    common::Vec2,
    dungeon::Dungeon,
    room::RoomId,
    view::{grid::Grid, View},
};
pub use commands::StateCommand;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::events::StateEvent;

pub enum EditMode {
    Select,
    AppendRoom,
}

pub struct CursorState {
    pub pos: Vec2<f64>,
}

pub struct State {
    pub dungeon: Dungeon,
    pub grid: Grid,
    pub view: View,
    pub cursor: CursorState,
    pub mode: EditMode,
    pub active_room_id: Option<RoomId>,
}
pub struct StateController {
    pub state: State,
    subscribers: HashMap<StateEvent, Vec<Rc<RefCell<dyn StateEventSubscriber>>>>,
    any_subscribers: Vec<Rc<RefCell<dyn StateEventSubscriber>>>,
    cmd_subscribers: Vec<Rc<RefCell<dyn StateCommandSubscriber>>>,
}

#[derive(Debug)]
pub struct StateCommandData {
    pub name: String,
    pub data: serde_json::Value,
}

pub trait StateEventSubscriber {
    fn on_state_event(&mut self, state: &mut State, event: StateEvent) -> Vec<StateCommand>;
}

pub trait StateCommandSubscriber {
    fn on_cmd_event(&mut self, state: &mut State, cmd: StateCommand);
}

impl State {
    pub fn new() -> Self {
        State {
            active_room_id: None,
            dungeon: Dungeon::new(),
            grid: Grid::new(),
            view: View::new(),
            mode: EditMode::Select,
            cursor: CursorState {
                pos: Vec2 { x: 0.0, y: 0.0 },
            },
        }
    }
}

impl StateController {
    pub fn new() -> Self {
        StateController {
            subscribers: HashMap::new(),
            any_subscribers: vec![],
            cmd_subscribers: vec![],
            state: State::new(),
        }
    }

    pub fn dungeon(&self) -> &Dungeon {
        &self.state.dungeon
    }

    pub fn apply(&mut self, command: StateCommand) {
        {
            let events = command.execute(&mut self.state);
            for e in events.iter() {
                self.notify(e.clone());
            }
        }
        for sub in self.cmd_subscribers.iter() {
            sub.borrow_mut()
                .on_cmd_event(&mut self.state, command.clone());
        }
    }

    pub fn subscribe(
        &mut self,
        event: StateEvent,
        subscriber: Rc<RefCell<dyn StateEventSubscriber>>,
    ) {
        self.subscribers.entry(event.clone()).or_default();
        self.subscribers.get_mut(&event).unwrap().push(subscriber);
    }

    pub fn subscribe_any(&mut self, subscriber: Rc<RefCell<dyn StateEventSubscriber>>) {
        self.any_subscribers.push(subscriber);
    }

    pub fn subscribe_cmds(&mut self, subscriber: Rc<RefCell<dyn StateCommandSubscriber>>) {
        self.cmd_subscribers.push(subscriber);
    }

    pub fn notify(&mut self, event: StateEvent) {
        let mut all_cmds: Vec<StateCommand> = vec![];
        match self.subscribers.get(&event) {
            None => (),
            Some(listeners) => {
                for listener in listeners {
                    all_cmds.append(
                        &mut listener
                            .borrow_mut()
                            .on_state_event(&mut self.state, event.clone()),
                    );
                }
            }
        }
        for listener in self.any_subscribers.iter() {
            all_cmds.append(
                &mut listener
                    .borrow_mut()
                    .on_state_event(&mut self.state, event.clone()),
            );
        }
        for cmd in all_cmds {
            self.apply(cmd);
        }
    }

    pub fn reset(&mut self) {
        self.state = State::new();
    }
}

impl CursorState {
    pub fn set_pos(&mut self, pos: Vec2<f64>) {
        self.pos = pos
    }
}
