use serde::{Serialize, Deserialize};
pub type Rgb = (u8, u8, u8);

#[derive(Serialize, Deserialize, Debug)]
pub enum Palette {
    White,
    LightGray,
    DarkGray,
    Black
}

impl Palette {
    pub fn from_u8(value: u8) -> Self {
        match value & 0b11 {
            0 => Palette::White,
            1 => Palette::LightGray,
            2 => Palette::DarkGray,
            3 => Palette::Black,
            _ => panic!("Invalid color: {}", value)
        }
    }

    pub fn into_rgb(&self) -> Rgb {
        match self {
            Palette::White => (255, 255, 255),
            Palette::LightGray => (192, 192, 192),
            Palette::DarkGray => (96, 96, 96),
            Palette::Black => (0, 0, 0)
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

impl Default for Palette {
    fn default() -> Self {
        Palette::White
    }
}