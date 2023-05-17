use crate::constants::gpu::*;
use serde::{Deserialize, Serialize};

pub type Rgb = (u8, u8, u8);

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Palette {
    #[default]
    White,
    LightGray,
    DarkGray,
    Black,
}

impl Palette {
    pub fn from_u8(value: u8) -> Self {
        match value & 0b11 {
            0 => Palette::White,
            1 => Palette::LightGray,
            2 => Palette::DarkGray,
            3 => Palette::Black,
            _ => panic!("Invalid color: {}", value),
        }
    }

    pub fn into_rgb(&self, use_green_colors: bool) -> Rgb {
        match (self, use_green_colors) {
            (Palette::White, false) => RGB_WHITE,
            (Palette::LightGray, false) => RGB_LIGHT_GRAY,
            (Palette::DarkGray, false) => RGB_DARK_GRAY,
            (Palette::Black, false) => RGB_BLACK,
            (Palette::White, true) => RGB_LIGHTEST_GREEN,
            (Palette::LightGray, true) => RGB_LIGHT_GREEN,
            (Palette::DarkGray, true) => RGB_DARK_GREEN,
            (Palette::Black, true) => RGB_DARKEST_GREEN,
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            Palette::White => 0,
            Palette::LightGray => 1,
            Palette::DarkGray => 2,
            Palette::Black => 3,
        }
    }
}
