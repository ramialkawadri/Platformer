use specs::prelude::*;
use specs::NullStorage;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(NullStorage)]
pub struct KeyboardControlled;
