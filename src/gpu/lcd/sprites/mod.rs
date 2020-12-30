use crate::{addresses::gpu::sprite::*, constants::gpu::RGB_WHITE};
use crate::constants::lcd::*;
use crate::constants::sprites::*;
use super::Lcd;
use sprite_attributes::SpriteAttributes;
use sprite_info::SpriteInfo;

pub mod sprite_attributes;
pub mod sprite_info;

impl Lcd {
    pub fn render_sprites(&mut self) {
        for sprite in 0..SPRITE_NUMBER {
            let index = (sprite * SPRITE_SIZE_BYTES) as u16;
            let sprite_info = self.get_sprite_info(index);
            let sprite_attributes = SpriteAttributes(sprite_info.attributes);
            let sprite_size = self.control.get_sprite_size();
            if !(self.line_number >= sprite_info.y_position && self.line_number < sprite_info.y_position + sprite_size) {
                return;
            }

            let line = self.get_line(sprite_size, &sprite_info, &sprite_attributes);
            let data_address = (TILE_MEMORY_REGION + (sprite_info.tile_location * TILE_MEMORY_OFFSET)) + line as u16;
            let pixel_low = self.read(data_address);
            let pixel_high = self.read(data_address + 1);

            for tile_bit in TILE_BITS..=0 {
                let color_number = self.get_sprite_color_number(
                    pixel_low,
                    pixel_high,
                    &sprite_attributes,
                    tile_bit
                );

                let color = if sprite_attributes.palette_number() {
                    self.obj_palette_1_data.get_color(color_number)
                } else {
                    self.obj_palette_0_data.get_color(color_number)
                };

                if color == RGB_WHITE {
                    continue;
                }

                let x = (0 - tile_bit) + TILE_BITS;
                self.screen.set_pixel(self.line_number as u16, x as u16, color)
            }
        }
    }
    
    fn get_sprite_info(&self, index: u16) -> SpriteInfo {
        let y_position = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index) - SPRITE_Y_OFFSET;
        let x_position = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index + 1) - SPRITE_X_OFFSET;
        let tile_location = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index + 2) as u16;
        let attributes = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index + 2);

        SpriteInfo {
            attributes,
            tile_location,
            x_position,
            y_position
        }
    }

    fn get_line(
        &self,
        sprite_size: u8,
        sprite_info: &SpriteInfo,
        sprite_attributes: &SpriteAttributes
    ) -> i16 {
        let mut line = (self.line_number - sprite_info.y_position) as i16;
        if sprite_attributes.y_flip() {
            line -= sprite_size as i16;
            line *= -1;
        }

        line *= 2;
        line
    }

    fn get_sprite_color_number(
        &self,
        pixel_low: u8,
        pixel_high: u8,
        sprite_attributes: &SpriteAttributes,
        tile_bit: i32
    ) -> u8 {
        let mut color_bit = tile_bit;
        if sprite_attributes.x_flip() {
            color_bit -= 7;
            color_bit *= -1;
        }

        let color_low = (pixel_low >> color_bit) & 0b1;
        let color_high = (pixel_high >> color_bit) & 0b1;
        (color_high << 1) | color_low
    }
}