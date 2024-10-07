use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use specs::prelude::*;
use specs::ReadStorage;

use crate::components::position::Position;
use crate::components::sprite::Sprite;

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

    let (width, height) = canvas.output_size()?;
    canvas.copy(&textures[0], Rect::new(0, 0, 480, 272), Rect::new(0, 0, width, height))?;

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
