pub mod lcd;
pub mod video_ram;

#[cfg(test)]
mod tests;

use crate::addresses::gpu::lcd::*;
use crate::addresses::gpu::video_ram::*;
use super::interrupts::lcd_interrupt::LcdInterruptResult;
use lcd::Lcd;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Gpu {
    lcd: Lcd,
    video_ram: video_ram::VideoRam
}

impl Gpu {
    pub fn clock(&mut self, cycles: u16) -> LcdInterruptResult {
        self.lcd.clock(cycles)
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL..=LCD_BG_PALETTE_DATA => self.lcd.read(address),
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.read(address),
            _ => panic!("Invalid GPU address 0x{:4X}", address)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            LCD_CONTROL..=LCD_BG_PALETTE_DATA => self.lcd.write(address, data),
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.write(address, data),
            _ => panic!("Invalid GPU address 0x{:4X}", address)
        }
    }
}