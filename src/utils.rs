use sdl2::rect::Rect;

use crate::components::sprite::Sprite;

pub fn divide_sheet_to_sprites(
    sprite_width: u32,
    sheet_width: u32,
    sheet_height: u32,
    spritesheet: usize,
) -> Vec<Sprite> {
    let mut output = Vec::new();

    let mut i = 0u32;
    while i < sheet_width {
        let region = Rect::new(i as i32, 0, sprite_width, sheet_height);
        output.push(Sprite { spritesheet, region, flip_horizontal: false });
        i += sprite_width;
    }

    return output;
}
