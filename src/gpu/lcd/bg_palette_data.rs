use super::palette::{Palette, Rgb};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct BgPaletteData {
    pub color_0: Palette,
    pub color_1: Palette,
    pub color_2: Palette,
    pub color_3: Palette,
}

impl BgPaletteData {
    pub fn from_u8(value: u8) -> Self {
        BgPaletteData {
            color_0: Palette::from_u8(value & 0b11),
            color_1: Palette::from_u8((value >> 2) & 0b11),
            color_2: Palette::from_u8((value >> 4) & 0b11),
            color_3: Palette::from_u8((value >> 6) & 0b11),
        }
    }

    pub fn get_color(&self, color_number: u8) -> Rgb {
        let palette = match color_number {
            0 => &self.color_0,
            1 => &self.color_1,
            2 => &self.color_2,
            3 => &self.color_3,
            _ => panic!("Invalid color number {}", color_number),
        };

        palette.into_rgb()
    }

    pub fn into_u8(&self) -> u8 {
        let nibble_3: u8 = self.color_3.into_u8();
        let nibble_2: u8 = self.color_2.into_u8();
        let nibble_1: u8 = self.color_1.into_u8();
        let nibble_0: u8 = self.color_0.into_u8();
        (nibble_3 << 6) | (nibble_2 << 4) | (nibble_1 << 2) | nibble_0
    }
}
