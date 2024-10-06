use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use specs::ReadStorage;

use crate::components::sprite::Sprite;
use crate::core::Position;

type SystemData<'a> = (
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, Position>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    textures: &[Texture],
    (sprites, positions): SystemData
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for (sprite, position) in (&sprites, &positions).join() {
        let dst = Rect::from_center(
            Point::new(position.x as i32, position.y as i32),
            sprite.region.width(),
            sprite.region.height());

        canvas.copy_ex(
            &textures[sprite.spritesheet],
            sprite.region,
            dst,
            0f64,
            None,
            sprite.flip_horizontal,
            false)?;
    }

    canvas.present();
    Ok(())
}
