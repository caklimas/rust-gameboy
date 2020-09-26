use serde::{Serialize, Deserialize};
pub type Rgb = (u8, u8, u8);

#[derive(Serialize, Deserialize, Debug)]
pub enum BackgroundColor {
    White,
    LightGray,
    DarkGray,
    Black
}

impl BackgroundColor {
    pub fn from_u8(value: u8) -> Self {
        match value & 0b11 {
            0 => BackgroundColor::White,
            1 => BackgroundColor::LightGray,
            2 => BackgroundColor::DarkGray,
            3 => BackgroundColor::Black,
            _ => panic!("Invalid color: {}", value)
        }
    }

    pub fn into_rgb(&self) -> Rgb {
        match self {
            BackgroundColor::White => (255, 255, 255),
            BackgroundColor::LightGray => (192, 192, 192),
            BackgroundColor::DarkGray => (96, 96, 96),
            BackgroundColor::Black => (0, 0, 0)
        }
    }

    pub fn into_u8(&self) -> u8 {
        match self {
            BackgroundColor::White => 0,
            BackgroundColor::LightGray => 1,
            BackgroundColor::DarkGray => 2,
            BackgroundColor::Black => 3,
        }
    }
}

impl Default for BackgroundColor {
    fn default() -> Self {
        BackgroundColor::White
    }
}