use sdl2::rect::Rect;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
    pub flip_horizontal: bool,
}
