use specs::prelude::*;

use crate::physics::components::speed::Speed;
use crate::events::movement_event::{Direction, MovementEvent};

pub struct MovementEventHandler;

const PLAYER_SPEED: f32 = 250f32;

impl<'a> System<'a> for MovementEventHandler {
    type SystemData = (
        ReadExpect<'a, Option<MovementEvent>>,
        // TODO: join with keyboard controlled!
        WriteStorage<'a, Speed>,
    );

    fn run(&mut self, (movement_event, mut speeds): Self::SystemData) {
        let direction = match &*movement_event {
            None => return,
            Some(movement_event) => movement_event.0.clone(),
        };

        for speed in (&mut speeds).join() {
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
