mod commands;
mod edit_mode;
pub mod events;

use crate::{
    chamber::{Chamber, ChamberId},
    common::Vec2,
    door::{Door, DoorId},
    dungeon::Dungeon,
    view::{grid::Grid, View},
};
pub use commands::StateCommand;
pub use edit_mode::EditMode;
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
    pub mode: EditMode,
    pub active_chamber_id: Option<ChamberId>,
    pub active_door_id: Option<DoorId>,
}
pub struct StateController {
    pub state: State,
    subscribers: HashMap<StateEvent, Vec<Rc<RefCell<dyn StateEventSubscriber>>>>,
    any_subscribers: Vec<Rc<RefCell<dyn StateEventSubscriber>>>,
    cmd_subscribers: Vec<Rc<RefCell<dyn StateCommandSubscriber>>>,
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
            active_chamber_id: None,
            active_door_id: None,
            dungeon: Dungeon::new(),
            grid: Grid::new(),
            view: View::new(),
            mode: EditMode::Select,
            cursor: CursorState {
                pos: Vec2 { x: 0.0, y: 0.0 },
            },
        }
    }

    pub fn active_chamber(&self) -> Option<&Chamber> {
        match self.active_chamber_id {
            Some(chamber_id) => self.dungeon.chamber(chamber_id),
            None => None,
        }
    }

    pub fn active_door(&self) -> Option<&Door> {
        match self.active_door_id {
            Some(door_id) => self.dungeon.door(door_id),
            None => None,
        }
    }

    pub fn cursor_world_pos(&self) -> Vec2<f64> {
        self.cursor.pos + self.view.world_min().into()
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

    pub fn apply_silent(&mut self, command: StateCommand) {
        command.execute(&mut self.state);
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
        self.notify(StateEvent::Reset);
    }

    pub fn reload(&mut self) {
        self.notify(StateEvent::Reload);
    }
}

impl CursorState {
    pub fn set_pos(&mut self, pos: Vec2<f64>) {
        self.pos = pos
    }
}
