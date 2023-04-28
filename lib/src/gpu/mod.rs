pub mod lcd;
pub mod video_oam;
pub mod video_ram;

#[cfg(test)]
mod tests;

use crate::addresses::gpu::lcd::*;
use crate::addresses::gpu::sprite::*;
use crate::addresses::gpu::video_ram::*;
use crate::mmu::interrupts::lcd_interrupt::LcdInterruptResult;
use lcd::Lcd;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Gpu {
    pub lcd: Lcd,
}

impl Gpu {
    pub fn clock(&mut self, cycles: u16) -> LcdInterruptResult {
        self.lcd.clock(cycles)
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL..=LCD_LYC
            | LCD_BG_PALETTE_DATA..=LCD_WINDOW_X
            | VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER
            | SPRITE_ATTRIBUTE_TABLE_LOWER..=SPRITE_ATTRIBUTE_TABLE_UPPER => self.lcd.read(address),
            _ => panic!("Invalid GPU address 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            LCD_CONTROL..=LCD_LYC
            | LCD_BG_PALETTE_DATA..=LCD_WINDOW_X
            | VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER
            | SPRITE_ATTRIBUTE_TABLE_LOWER..=SPRITE_ATTRIBUTE_TABLE_UPPER => {
                self.lcd.write(address, data)
            }
            _ => panic!("Invalid GPU address 0x{:4X}", address),
        }
    }
}
