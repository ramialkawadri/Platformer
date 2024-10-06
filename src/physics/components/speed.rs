use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}
