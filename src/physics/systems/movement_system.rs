use specs::prelude::*;

use crate::core::{DeltaTime, Position};
use crate::physics::components::speed::Speed;

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        ReadStorage<'a, Speed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (delta_time, speeds, mut positions): Self::SystemData) {
        for (speed, position) in (&speeds, &mut positions).join() {
            position.x += delta_time.0 * speed.x;
            position.y += delta_time.0 * speed.y;
        }
    }
}
