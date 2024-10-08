use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

impl Speed {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
