use crate::addresses::gpu::sprite::*;
use crate::constants::sprites::*;
use super::Lcd;
use super::sprite_attributes::SpriteAttributes;

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

            let mut line = (self.line_number - sprite_info.y_position) as i16;
            if sprite_attributes.y_flip() {
                line -= sprite_size as i16;
                line *= -1;
            }

            line *= 2;
        }
    }
    
    fn get_sprite_info(&self, index: u16) -> SpriteInfo {
        let y_position = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index) - SPRITE_Y_OFFSET;
        let x_position = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index + 1) - SPRITE_X_OFFSET;
        let tile_location = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index + 2);
        let attributes = self.read(SPRITE_ATTRIBUTE_TABLE_LOWER + index + 2);

        SpriteInfo {
            attributes,
            tile_location,
            x_position,
            y_position
        }
    }
}

struct SpriteInfo {
    pub attributes: u8,
    pub tile_location: u8,
    pub x_position: u8,
    pub y_position: u8
}