use super::palette::Palette;
use super::sprites::sprite_color::SpriteColor;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ObjPaletteData {
    color_0: Palette,
    color_1: Palette,
    color_2: Palette,
    color_3: Palette,
}

impl ObjPaletteData {
    pub fn from_u8(value: u8) -> Self {
        ObjPaletteData {
            color_0: Palette::White,
            color_1: Palette::from_u8((value >> 2) & 0b11),
            color_2: Palette::from_u8((value >> 4) & 0b11),
            color_3: Palette::from_u8((value >> 6) & 0b11),
        }
    }

    pub fn get_color(&self, color_number: u8, use_green_color: bool) -> SpriteColor {
        let palette = match color_number {
            0 => &self.color_0, // sprite index 0 means transparent
            1 => &self.color_1,
            2 => &self.color_2,
            3 => &self.color_3,
            _ => panic!("Invalid color number {}", color_number),
        };

        SpriteColor {
            color: palette.into_rgb(use_green_color),
            index: color_number,
        }
    }

    pub fn into_u8(&self) -> u8 {
        let nibble_3: u8 = self.color_3.into_u8();
        let nibble_2: u8 = self.color_2.into_u8();
        let nibble_1: u8 = self.color_1.into_u8();
        (nibble_3 << 6) | (nibble_2 << 4) | (nibble_1 << 2)
    }
}
