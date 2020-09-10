pub mod boot_rom;
pub mod high_ram;
pub mod memory_sizes;
pub mod ram;
pub mod work_ram;
pub mod video_ram;

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use super::addresses::boot_rom::*;

#[derive(Serialize, Deserialize, Default)]
pub struct MemoryManagementUnit {
    boot_rom: boot_rom::BootRom,
    ram: ram::Ram
}

impl MemoryManagementUnit {
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;

        (high << 8) | low
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            BOOT_ROM_LOWER..=BOOT_ROM_UPPER => self.boot_rom.read(address),
            _ => self.ram.read(address)
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
            BOOT_ROM_LOWER..=BOOT_ROM_UPPER => self.boot_rom.write(address, data),
            _ => self.ram.write(address, data)
        }
    }
}