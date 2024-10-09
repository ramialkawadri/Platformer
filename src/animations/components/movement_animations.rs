use specs::prelude::*;
use specs::VecStorage;
use specs_derive::Component;

use crate::core::components::sprite::Sprite;
use crate::core::direction::Direction;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimations {
    pub movement_frames: Vec<Sprite>,
    pub idle_frames: Vec<Sprite>,
    pub current_direction: Direction,
}

impl MovementAnimations {
    pub fn new(
        movement_frames: Vec<Sprite>,
        idle_frames: Vec<Sprite>,
        current_direction: Direction,
    ) -> Self {
        MovementAnimations {
            movement_frames,
            idle_frames,
            current_direction,
        }
    }
}
