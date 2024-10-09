use specs::prelude::*;

use crate::animations::components::animation::Animation;
use crate::core::components::sprite::Sprite;
use crate::core::delta_time::DeltaTime;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        ReadExpect<'a, DeltaTime>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Sprite>,
    );

    fn run(
        &mut self,
        (delta_time, mut animations, mut sprites): Self::SystemData
    ) {
        for (anim, sprite) in (&mut animations, &mut sprites).join() {
            anim.elapsed_time += delta_time.0;

            let num_of_frames = anim.frames.len();
            if anim.elapsed_time >= 1f32 / num_of_frames as f32 {
                anim.elapsed_time = 0f32;
                anim.current_frame = (anim.current_frame + 1) % num_of_frames;
                *sprite = anim.frames[anim.current_frame].clone();
            }
        }
    }
}
