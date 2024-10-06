use specs::VecStorage;
use specs_derive::Component;
use specs::prelude::*;

use crate::events::movement_event::Direction;
use crate::components::sprite::Sprite;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MoveableEntity {
    pub movement_frames: Vec<Sprite>,
    pub idle_frames: Vec<Sprite>,
    pub current_direction: Direction,
}
