use specs::prelude::*;
use specs_derive::Component;

use crate::core::components::sprite::Sprite;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Animation {
    pub frames: Vec<Sprite>,
    pub current_frame: usize,
    pub elapsed_time: f32,
}

impl Animation {
    pub fn new(frames: Vec<Sprite>, current_frame: usize, elapsed_time: f32) -> Self {
        Self { frames, current_frame, elapsed_time }
    }
}
