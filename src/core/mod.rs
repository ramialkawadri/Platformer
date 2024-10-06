use specs::VecStorage;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Default)]
pub struct DeltaTime(pub f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

