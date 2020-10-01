use serde::{Serialize, Deserialize};
use crate::addresses::gpu::lcd::*;

pub mod bg_color;
pub mod bg_palette_data;
pub mod lcd_control;
pub mod lcd_mode;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Lcd {
    bg_palette_data: bg_palette_data::BgPaletteData,
    control: lcd_control::LcdControl,
    scroll_y: u8
}

impl Lcd {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL => self.control.get(),
            LCD_SCROLL_Y => self.scroll_y,
            LCD_BG_PALETTE_DATA => self.bg_palette_data.into_u8(),
            _ => panic!("Invalid lcd address: 0x{:4X}", address)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            LCD_CONTROL => self.control.set(data),
            LCD_SCROLL_Y => self.scroll_y = data,
            LCD_BG_PALETTE_DATA => self.bg_palette_data = bg_palette_data::BgPaletteData::from_u8(data),
            _ => panic!("Invalid lcd address: 0x{:4X}", address)
        }
    }
}