pub mod high_ram;
pub mod memory;
pub mod memory_sizes;
pub mod work_ram;
pub mod video_ram;

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use super::addresses::video_ram::*;

#[derive(Serialize, Deserialize, Default)]
pub struct MemoryManagementUnit {
    video_ram: video_ram::VideoRam,
    work_ram: work_ram::WorkRam
}

impl MemoryManagementUnit {
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;

        (high << 8) | low
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => {
                self.video_ram.read(address)
            },
            _ => {
                println!("Invalid address 0x{:4X}", address);
                0
            }
        }
    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        let low = (data & 0xFF) as u8;
        let high = ((data & 0xFF00) >> 8) as u8;

        self.write_byte(address, low);
        self.write_byte(address + 1, high);
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => {
                self.video_ram.write(address, data);
            },
            _ => {
                println!("Invalid address 0x{:4X}", address);
            }
        }
    }
}