use specs::prelude::*;

use crate::components::animation::Animation;
use crate::components::moveable_entity::MoveableEntity;
use crate::components::sprite::Sprite;
use crate::events::movement_event::{Direction, MovementEvent};

pub struct MovementAnimator;

impl<'a> System<'a> for MovementAnimator {
    type SystemData = (
        ReadExpect<'a, Option<MovementEvent>>,
        WriteStorage<'a, MoveableEntity>,
        WriteStorage<'a, Animation>,
    );

    fn run(&mut self, (movement_event, mut moveable_entities, mut animations): Self::SystemData) {
        let movement_event = match &*movement_event {
            Some(movement_event) => movement_event,
            None => return,
        };

        for (moveable_entity, animation) in (&mut moveable_entities, &mut animations).join() {
            if moveable_entity.current_direction == movement_event.0 {
                continue;
            }
            moveable_entity.current_direction = movement_event.0.clone();

            let mut frames;
            if movement_event.0 == Direction::Stop {
                frames = moveable_entity.idle_frames.clone();
            } else {
                frames = moveable_entity.movement_frames.clone();
            }

            if movement_event.0 == Direction::Left {
                frames = frames
                    .iter()
                    .map(|frame| Sprite {
                        flip_horizontal: true,
                        ..*frame
                    })
                    .collect();
            }

            *animation = Animation {
                frames,
                current_frame: 0,
                elapsed_time: 0f32,
            }
        }
    }
}
