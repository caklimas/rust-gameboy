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
use super::addresses::cartridge::*;
use super::cartridge::Cartridge;

#[derive(Serialize, Deserialize, Default)]
pub struct Mmu {
    boot_rom: boot_rom::BootRom,
    cartridge: Option<Cartridge>,
    ram: ram::Ram,
    running_boot_rom: bool
}

impl Mmu {
    pub fn new(cartridge: Cartridge) -> Self {
        Mmu {
            boot_rom: Default::default(),
            cartridge: Some(cartridge),
            ram: Default::default(),
            running_boot_rom: false
        }
    }
    
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;

        (high << 8) | low
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match (address, self.running_boot_rom) {
            (BOOT_ROM_LOWER..=BOOT_ROM_UPPER, true) => self.boot_rom.read(address),
            (CART_ROM_LOWER..=CART_ROM_UPPER, _) => self.read_mbc_rom(address),
            (CART_EXTERNAL_RAM_LOWER..=CART_EXTERNAL_RAM_UPPER, _) => self.read_mbc_ram(address),
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
        match (address, self.running_boot_rom) {
            (BOOT_ROM_LOWER..=BOOT_ROM_UPPER, true) => self.boot_rom.write(address, data),
            (CART_ROM_LOWER..=CART_ROM_UPPER, _) => self.write_mbc_rom(address, data),
            (CART_EXTERNAL_RAM_LOWER..=CART_EXTERNAL_RAM_UPPER, _) => self.write_mbc_ram(address, data),
            _ => self.ram.write(address, data)
        }
    }

    fn read_mbc_ram(&self, address: u16) -> u8 {
        if let Some(ref c) = self.cartridge {
            if let Some(ref m) = c.mbc {
                return m.read_ram(address);
            }
        }

        0
    }

    fn read_mbc_rom(&self, address: u16) -> u8 {
        if let Some(ref c) = self.cartridge {
            if let Some(ref m) = c.mbc {
                return m.read_rom(address);
            }
        }

        0
    }

    fn write_mbc_ram(&mut self, address: u16, data: u8) {
        if let Some(ref mut c) = self.cartridge {
            if let Some(ref mut m) = c.mbc {
                m.write_ram(address, data);
            }
        }
    }

    fn write_mbc_rom(&mut self, address: u16, data: u8) {
        if let Some(ref mut c) = self.cartridge {
            if let Some(ref mut m) = c.mbc {
                m.write_rom(address, data);
            }
        }
    }
}