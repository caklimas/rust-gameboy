use serde::{Serialize, Deserialize};
use super::palette::{Palette, Rgb};

#[derive(Serialize, Deserialize, Default)]
pub struct ObjPaletteData {
    color_1: Palette,
    color_2: Palette,
    color_3: Palette
}

impl ObjPaletteData {
    pub fn from_u8(value: u8) -> Self {
        ObjPaletteData {
            color_1: Palette::from_u8((value >> 2) & 0b11),
            color_2: Palette::from_u8((value >> 4) & 0b11),
            color_3: Palette::from_u8((value >> 6) & 0b11)
        }
    }

    pub fn get_color(&self, color_number: u8) -> Rgb {
        let palette = match color_number {
            1 => &self.color_1,
            2 => &self.color_2,
            3 => &self.color_3,
            _ => panic!("Invalid color number {}", color_number)
        };

        palette.into_rgb()
    }

    pub fn into_u8(&self) -> u8 {
        let nibble_3: u8 = self.color_3.into_u8();
        let nibble_2: u8 = self.color_2.into_u8();
        let nibble_1: u8 = self.color_1.into_u8();
        (nibble_3 << 6) | (nibble_2 << 4) | (nibble_1 << 2)
    }
}