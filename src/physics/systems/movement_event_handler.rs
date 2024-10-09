use specs::prelude::*;

use crate::core::components::keyboard_controlled::KeyboardControlled;
use crate::core::direction::Direction;
use crate::physics::components::speed::Speed;
use crate::events::movement_event::MovementEvent;

pub struct MovementEventHandler;

const PLAYER_SPEED: f32 = 250f32;

impl<'a> System<'a> for MovementEventHandler {
    type SystemData = (
        ReadExpect<'a, Option<MovementEvent>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Speed>,
    );

    fn run(&mut self, (movement_event, keyboard_controlled, mut speeds): Self::SystemData) {
        let direction = match &*movement_event {
            None => return,
            Some(movement_event) => movement_event.0.clone(),
        };

        for (speed, _) in (&mut speeds, &keyboard_controlled).join() {
            match direction {
                Direction::Left => {
                    speed.x = -PLAYER_SPEED;
                }
                Direction::Right => {
                    speed.x = PLAYER_SPEED;
                }
                Direction::Stop => {
                    speed.x = 0f32;
                }
            }
        }

    }
}
