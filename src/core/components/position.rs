use specs::VecStorage;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position { x, y }
    }
}

