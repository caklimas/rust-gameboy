use crate::{
    addresses::mbc::mbc3::{ROM_BANK_0_LOWER, ROM_BANK_1_7F_LOWER},
    cartridge::cartridge_header::CartridgeHeader,
};

use super::Mbc;

pub struct Mbc3 {
    ram: Vec<u8>,
    ram_bank_number: u8,
    ram_enabled: bool,
    rom: Vec<u8>,
    rom_bank_number: u8,
    ram_enabled: bool,
}

impl Mbc3 {
    pub fn new(header: &CartridgeHeader, data: Vec<u8>) -> Self {
        Self {
            ram: vec![0; header.ram_size.get_size()],
            ram_bank_number: 0x00,
            ram_enabled: false,
            rom: data,
            rom_bank_number: 0x01,
        }
    }

    fn write_ram_enabled(&mut self, data: u8) {
        self.ram_enabled = data == ENABLE_RAM;
    }
}

impl Mbc for Mbc3 {
    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }

        todo!()
    }

    fn read_rom(&self, address: u16) -> u8 {
        match address {
            ROM_BANK_0_LOWER..=ROM_BANK_0_LOWER => self.rom[address as usize],
            ROM_BANK_1_7F_LOWER..=ROM_BANK_1_7F_LOWER => {
                let offset = self.rom_bank_number as u16 * ROM_BANK_1_7F_LOWER;
                let index = address - ROM_BANK_1_7F_LOWER + offset;
                self.rom[index as usize]
            }
            _ => panic!("Invalid MBC3 address"),
        }
    }

    fn write_ram(&mut self, address: u16, data: u8) {
        todo!()
    }

    fn write_rom(&mut self, address: u16, data: u8) {
        todo!()
    }
}
