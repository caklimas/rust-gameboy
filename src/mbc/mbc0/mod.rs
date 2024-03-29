#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use crate::mmu::memory_sizes::*;

#[derive(Serialize, Deserialize)]
pub struct Mbc0 {
    #[serde(skip)]
    ram: Vec<u8>,
    rom: Vec<u8>,
}

impl Mbc0 {
    pub fn new(data: Vec<u8>) -> Self {
        Mbc0 {
            ram: vec![0; KILOBYTES_8 as usize],
            rom: data,
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        let index = self.get_ram_index(address);
        self.ram[index]
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        let index = self.get_rom_index(address);
        self.rom[index]
    }

    pub fn write_ram(&mut self, address: u16, data: u8) {
        let index = self.get_ram_index(address);
        self.ram[index] = data;
    }

    pub fn write_rom(&mut self, _address: u16, _data: u8) {}

    pub fn has_battery(&self) -> bool {
        false
    }

    fn get_ram_index(&self, address: u16) -> usize {
        (address % KILOBYTES_8) as usize
    }

    fn get_rom_index(&self, address: u16) -> usize {
        (address % KILOBYTES_32) as usize
    }
}
