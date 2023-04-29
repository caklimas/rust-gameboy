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

    pub fn into_rgb(&self) -> Rgb {
        match self {
            Palette::White => RGB_WHITE,
            Palette::LightGray => RGB_LIGHT_GRAY,
            Palette::DarkGray => RGB_DARK_GRAY,
            Palette::Black => RGB_BLACK,
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
