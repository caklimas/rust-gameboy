use super::Lcd;
use crate::addresses::gpu::sprite::SPRITE_ATTRIBUTE_TABLE_LOWER;
use crate::constants::gpu::WINDOW_X_OFFSET;
use crate::constants::lcd::*;
use crate::constants::screen::SCREEN_WIDTH;
use crate::constants::sprites::*;
use sprite_attributes::SpriteAttributes;
use sprite_info::SpriteInfo;

pub mod sprite_attributes;
pub mod sprite_color;
pub mod sprite_info;

impl Lcd {
    pub fn render_sprites(&mut self) {
        let sprite_size = self.control.get_sprite_size();
        for sprite in 0..SPRITE_NUMBER {
            let index = (SPRITE_NUMBER - 1) - sprite;
            let sprite_info = self.get_sprite_info(index as u16, sprite_size);
            let sprite_attributes = SpriteAttributes(sprite_info.attributes);
            let line = self.line_number as i32;
            if line < sprite_info.y_position
                || line >= sprite_info.y_position + (sprite_size as i32)
            {
                continue;
            }

            if sprite_info.x_position < -(WINDOW_X_OFFSET as i32)
                || sprite_info.x_position >= (SCREEN_WIDTH as i32)
            {
                continue;
            }

            let tile_y = self.get_line(sprite_size as i32, &sprite_info, &sprite_attributes);
            let data_address =
                (TILE_MEMORY_REGION + (sprite_info.tile_location * TILE_SIZE)) + tile_y;
            let pixel_low = self.read(data_address);
            let pixel_high = self.read(data_address + 1);

            for tile_bit in 0..=TILE_BITS {
                if sprite_info.x_position + tile_bit < 0
                    || sprite_info.x_position + tile_bit >= (SCREEN_WIDTH as i32)
                {
                    continue;
                }

                let color_number = self.get_sprite_color_number(
                    pixel_low,
                    pixel_high,
                    &sprite_attributes,
                    tile_bit,
                );

                let sprite_color = if sprite_attributes.palette_number() {
                    self.obj_palette_1_data.get_color(color_number)
                } else {
                    self.obj_palette_0_data.get_color(color_number)
                };

                let x = sprite_info.x_position + tile_bit;

                if sprite_color.index == 0 {
                    continue;
                }

                if !sprite_attributes.obj_priority() {
                    self.screen
                        .set_pixel(self.line_number as u16, x as u16, sprite_color.color);
                }
            }
        }
    }

    fn get_sprite_info(&self, index: u16, sprite_size: u8) -> SpriteInfo {
        let sprite_address = SPRITE_ATTRIBUTE_TABLE_LOWER + index * SPRITE_SIZE_BYTES;
        let y_position = self.read(sprite_address) as u16 as i32 - SPRITE_Y_OFFSET;
        let x_position = self.read(sprite_address + 1) as u16 as i32 - SPRITE_X_OFFSET;
        let tile_location = self.read(sprite_address + 2) as u16;
        let attributes = self.read(sprite_address + 3);

        SpriteInfo {
            attributes,
            tile_location: tile_location & (if sprite_size == 16 { 0xFE } else { 0xFF }),
            x_position,
            y_position,
        }
    }

    fn get_line(
        &self,
        sprite_size: i32,
        sprite_info: &SpriteInfo,
        sprite_attributes: &SpriteAttributes,
    ) -> u16 {
        let line = self.line_number as i32;
        let tile_y = if sprite_attributes.y_flip() {
            (sprite_size - 1 - (line - sprite_info.y_position)) as u16
        } else {
            (line - sprite_info.y_position) as u16
        };

        tile_y * 2
    }

    fn get_sprite_color_number(
        &self,
        pixel_low: u8,
        pixel_high: u8,
        sprite_attributes: &SpriteAttributes,
        tile_bit: i32,
    ) -> u8 {
        let mut color_bit = tile_bit;
        if !sprite_attributes.x_flip() {
            color_bit = TILE_BITS - color_bit;
        }

        let color_low = (pixel_low >> color_bit) & 0b1;
        let color_high = (pixel_high >> color_bit) & 0b1;
        (color_high << 1) | color_low
    }
}
