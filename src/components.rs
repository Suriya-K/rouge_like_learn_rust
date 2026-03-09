use bracket_lib::prelude::{FontCharType, RGB};
use specs::prelude::*;
use specs_derive::*;

use crate::xy_idx;

// if not use #[derive(Component)] marco then we will have to write manual component
// imple Component for Position {
//      type Storage = VecStorage<Self>;
// }
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderer {
    pub glyph: FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}

#[derive(Component)]
pub struct Movement {}

#[derive(Component, Debug)]
pub struct Player {}

impl Position {
    pub fn get_pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn get_idx(&self) -> usize {
        xy_idx(self.x, self.y)
    }
}

#[derive(Component)]
pub struct Room {
    pub room_number: i32,
}
