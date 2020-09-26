use serde::{Serialize, Deserialize};
use super::bg_color::BackgroundColor;

#[derive(Serialize, Deserialize, Default)]
pub struct BgPaletteData {
    color_0: BackgroundColor,
    color_1: BackgroundColor,
    color_2: BackgroundColor,
    color_3: BackgroundColor
}

impl BgPaletteData {
    pub fn from_u8(value: u8) -> Self {
        BgPaletteData {
            color_0: BackgroundColor::from_u8(value & 0b11),
            color_1: BackgroundColor::from_u8((value >> 2) & 0b11),
            color_2: BackgroundColor::from_u8((value >> 4) & 0b11),
            color_3: BackgroundColor::from_u8((value >> 6) & 0b11)
        }
    }

    pub fn into_u8(&self) -> u8 {
        let nibble_3: u8 = self.color_3.into_u8();
        let nibble_2: u8 = self.color_2.into_u8();
        let nibble_1: u8 = self.color_1.into_u8();
        let nibble_0: u8 = self.color_0.into_u8();
        (nibble_3 << 6) | (nibble_2 << 4) | (nibble_1 << 2) | nibble_0
    }
}