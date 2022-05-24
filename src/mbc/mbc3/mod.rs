use crate::cartridge::cartridge_header::CartridgeHeader;

use super::Mbc;

pub struct Mbc3 {
    ram_enabled: bool,
    rom: Vec<u8>,
}

impl Mbc3 {
    pub fn new(header: &CartridgeHeader, data: Vec<u8>) -> Self {
        Self {
            ram_enabled: false,
            rom: data,
        }
    }
}

impl Mbc for Mbc3 {
    fn read_ram(&self, address: u16) -> u8 {
        todo!()
    }

    fn read_rom(&self, address: u16) -> u8 {
        todo!()
    }

    fn write_ram(&mut self, address: u16, data: u8) {
        todo!()
    }

    fn write_rom(&mut self, address: u16, data: u8) {
        todo!()
    }
}
