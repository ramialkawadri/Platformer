use specs::prelude::*;
use specs::VecStorage;
use specs_derive::Component;

use crate::components::sprite::Sprite;
use crate::events::movement_event::Direction;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MoveableEntity {
    pub movement_frames: Vec<Sprite>,
    pub idle_frames: Vec<Sprite>,
    pub current_direction: Direction,
}

impl MoveableEntity {
    pub fn new(
        movement_frames: Vec<Sprite>,
        idle_frames: Vec<Sprite>,
        current_direction: Direction,
    ) -> Self {
        MoveableEntity {
            movement_frames,
            idle_frames,
            current_direction,
        }
    }
}
