use specs::prelude::*;
use specs_derive::Component;

use super::sprite::Sprite;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Animation {
    pub frames: Vec<Sprite>,
    pub current_frame: usize,
    pub elapsed_time: f32,
}
