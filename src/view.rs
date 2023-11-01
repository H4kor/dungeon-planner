pub mod add_room_button;
pub mod grid;
pub mod primitives;
pub mod room_edit;
pub mod room_list;

use crate::common::Vec2;

#[derive(Clone, Copy)]
pub struct View {
    offset: Vec2<i32>,
}

impl View {
    pub fn new() -> Self {
        Self {
            offset: Vec2 { x: 0, y: 0 },
        }
    }

    pub fn move_view(&mut self, by: Vec2<i32>) {
        self.offset += by
    }

    pub fn world_min(self) -> Vec2<i32> {
        self.offset
    }
}
