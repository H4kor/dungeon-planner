use crate::{
    common::Vec2,
    dungeon::Dungeon,
    view::{grid::Grid, View},
};

pub struct CursorState {
    pub pos: Vec2<f64>,
}

pub struct State {
    pub dungeon: Dungeon,
    pub grid: Grid,
    pub view: View,
    pub cursor_state: CursorState,
}

impl State {
    pub fn new(dungeon: Dungeon, grid: Grid, view: View) -> Self {
        State {
            dungeon: dungeon,
            grid: grid,
            view: view,
            cursor_state: CursorState {
                pos: Vec2 { x: 0.0, y: 0.0 },
            },
        }
    }
}

impl CursorState {
    pub fn set_pos(&mut self, pos: Vec2<f64>) {
        self.pos = pos
    }
}
