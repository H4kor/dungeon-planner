pub mod commands;
pub mod events;

use crate::{
    common::Vec2,
    dungeon::Dungeon,
    room::RoomId,
    view::{grid::Grid, View},
};
use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use self::events::StateEvent;

pub struct CursorState {
    pub pos: Vec2<f64>,
}

pub struct State {
    pub dungeon: Dungeon,
    pub grid: Grid,
    pub view: View,
    pub cursor: CursorState,
    pub active_room_id: Option<RoomId>,
}
pub struct StateController {
    pub state: State,
    subscribers: HashMap<StateEvent, Vec<Rc<RefCell<dyn StateSubscriber>>>>,
    any_subscribers: Vec<Rc<RefCell<dyn StateSubscriber>>>,
    cmd_subscribers: Vec<Rc<RefCell<dyn StateCommandSubscriber>>>,
}

#[derive(Debug)]
pub struct StateCommandData {
    pub name: String,
    pub data: serde_json::Value,
}

pub trait StateCommand {
    fn execute(&self, state: &mut State) -> Vec<StateEvent>;
    fn data(&self) -> StateCommandData;
}

pub trait StateSubscriber {
    fn on_state_event(
        &mut self,
        state: &mut State,
        event: StateEvent,
    ) -> Vec<RefCell<Box<dyn StateCommand>>>;
}

pub trait StateCommandSubscriber {
    fn on_cmd_event(&mut self, state: &mut State, cmd: &dyn StateCommand);
}

impl StateController {
    pub fn new(dungeon: Dungeon, grid: Grid, view: View) -> Self {
        StateController {
            subscribers: HashMap::new(),
            any_subscribers: vec![],
            cmd_subscribers: vec![],
            state: State {
                active_room_id: None,
                dungeon: dungeon,
                grid: grid,
                view: view,
                cursor: CursorState {
                    pos: Vec2 { x: 0.0, y: 0.0 },
                },
            },
        }
    }

    pub fn dungeon(&self) -> &Dungeon {
        &self.state.dungeon
    }

    pub fn apply(&mut self, command: RefCell<Box<dyn StateCommand>>) {
        {
            let events = command.borrow().execute(&mut self.state);
            for e in events.iter() {
                self.notify(e.clone());
            }
        }
        for sub in self.cmd_subscribers.iter() {
            sub.borrow_mut()
                .on_cmd_event(&mut self.state, command.borrow().as_ref());
        }
    }

    pub fn subscribe(&mut self, event: StateEvent, subscriber: Rc<RefCell<dyn StateSubscriber>>) {
        self.subscribers.entry(event.clone()).or_default();
        self.subscribers.get_mut(&event).unwrap().push(subscriber);
    }

    pub fn subscribe_any(&mut self, subscriber: Rc<RefCell<dyn StateSubscriber>>) {
        self.any_subscribers.push(subscriber);
    }

    pub fn subscribe_cmds(&mut self, subscriber: Rc<RefCell<dyn StateCommandSubscriber>>) {
        self.cmd_subscribers.push(subscriber);
    }

    pub fn notify(&mut self, event: StateEvent) {
        let mut all_cmds: Vec<RefCell<Box<dyn StateCommand>>> = vec![];
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
}

impl CursorState {
    pub fn set_pos(&mut self, pos: Vec2<f64>) {
        self.pos = pos
    }
}
