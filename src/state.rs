pub mod commands;
pub mod events;

use crate::{
    common::Vec2,
    dungeon::Dungeon,
    room::{Room, RoomId},
    view::{grid::Grid, View},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::events::StateEvent;

pub struct CursorState {
    pub pos: Vec2<f64>,
}

pub struct State {
    pub dungeon: Dungeon,
    pub grid: Grid,
    pub view: View,
    pub cursor: CursorState,
}
pub struct StateController {
    pub state: State,
    subscribers: HashMap<StateEvent, Vec<Rc<RefCell<dyn StateSubscriber>>>>,
    commands: Vec<Box<dyn StateCommand>>,
}

pub trait StateCommand {
    fn execute(&self, state: &mut State) -> Vec<StateEvent>;
}

pub trait StateSubscriber {
    fn on_state_event(
        &mut self,
        state: &mut State,
        event: StateEvent,
    ) -> Vec<Box<dyn StateCommand>>;
}

impl StateController {
    pub fn new(dungeon: Dungeon, grid: Grid, view: View) -> Self {
        StateController {
            subscribers: HashMap::new(),
            commands: vec![],
            state: State {
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

    pub fn apply(&mut self, command: Box<dyn StateCommand>) {
        let events = command.execute(&mut self.state);
        for e in events.iter() {
            self.notify(e.clone());
        }
    }

    pub fn subscribe(&mut self, event: StateEvent, subscriber: Rc<RefCell<dyn StateSubscriber>>) {
        self.subscribers.entry(event.clone()).or_default();
        self.subscribers.get_mut(&event).unwrap().push(subscriber);
    }

    pub fn notify(&mut self, event: StateEvent) {
        let listeners = self.subscribers.get(&event).unwrap();
        for listener in listeners {
            listener
                .borrow_mut()
                .on_state_event(&mut self.state, event.clone());
        }
    }
}

impl CursorState {
    pub fn set_pos(&mut self, pos: Vec2<f64>) {
        self.pos = pos
    }
}
