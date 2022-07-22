pub mod addresses;
pub mod banking_mode;

#[cfg(test)]
mod tests;

use crate::addresses::mbc::mbc1::*;
use crate::cartridge::cartridge_header::cartridge_type::CartridgeType;
use crate::cartridge::cartridge_header::CartridgeHeader;
use crate::mmu::memory_sizes::{KILOBYTES_16, KILOBYTES_8};
use banking_mode::BankingMode;
use serde::{Deserialize, Serialize};

pub const ENABLE_RAM: u8 = 0x0A;

#[derive(Serialize, Deserialize)]
pub struct Mbc1 {
    bank_mode: BankingMode,
    #[serde(skip)]
    ram: Vec<u8>,
    ram_bank_number: u8,
    ram_enabled: bool,
    rom: Vec<u8>,
    rom_bank_number: u8,
    has_battery: bool,
}

impl Mbc1 {
    pub fn new(header: &CartridgeHeader, data: Vec<u8>) -> Self {
        Mbc1 {
            bank_mode: BankingMode::Rom,
            ram: vec![0; header.ram_size.get_size()],
            ram_bank_number: 0x00,
            ram_enabled: false,
            rom: data,
            rom_bank_number: 0x01,
            has_battery: header.cartridge_type == CartridgeType::Mbc1RamBattery,
        }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }

        let index = self.get_ram_index(address);
        self.ram[index]
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        let index = if address < KILOBYTES_16 {
            address as usize
        } else {
            (self.rom_bank_number as usize * KILOBYTES_16 as usize)
                | (address as usize % KILOBYTES_16 as usize)
        };

        self.rom[index]
    }

    pub fn write_ram(&mut self, address: u16, data: u8) {
        if !self.ram_enabled {
            return;
        }

        let index = self.get_ram_index(address);
        self.ram[index as usize] = data;
    }

    pub fn write_rom(&mut self, address: u16, data: u8) {
        match address {
            RAM_ENABLE_LOWER..=RAM_ENABLE_UPPER => self.write_ram_enabled(data),
            ROM_BANK_NUMBER_LOWER..=ROM_BANK_NUMBER_UPPER => self.write_rom_bank_number_lower(data),
            RAM_BANK_NUMBER_LOWER..=RAM_BANK_NUMBER_UPPER => self.write_bank_number(data),
            BANKING_MODE_SELECT_LOWER..=BANKING_MODE_SELECT_UPPER => self.write_bank_mode(data),
            _ => (),
        }
    }

    pub fn set_ram(&mut self, data: Vec<u8>) {
        if !self.has_battery {
            return;
        }

        self.ram = data;
    }

    pub fn get_ram(&self) -> Vec<u8> {
        self.ram.clone()
    }

    pub fn has_battery(&self) -> bool {
        self.has_battery
    }

    fn get_ram_index(&self, address: u16) -> usize {
        let ram_bank = match self.bank_mode {
            BankingMode::Ram => self.ram_bank_number,
            BankingMode::Rom => 0,
        } as u16;

        ((KILOBYTES_8 * ram_bank) | (address % KILOBYTES_8)) as usize
    }

    fn write_bank_mode(&mut self, data: u8) {
        self.bank_mode = BankingMode::new(data);
    }

    fn write_bank_number(&mut self, data: u8) {
        match self.bank_mode {
            BankingMode::Rom => {
                let upper = (data & 0b11) << 5;
                let lower = self.rom_bank_number & 0b1_1111;
                self.rom_bank_number = upper | lower;
            }
            BankingMode::Ram => {
                self.ram_bank_number = data & 0b11;
            }
        }
    }

    fn write_ram_enabled(&mut self, data: u8) {
        self.ram_enabled = data == ENABLE_RAM;
    }

    fn write_rom_bank_number_lower(&mut self, data: u8) {
        let bank_number_upper = self.rom_bank_number & 0b110_0000;
        let bank_number_lower = match data & 0b1_1111 {
            0 => 1,
            d => d,
        };

        self.rom_bank_number = bank_number_upper | bank_number_lower;
    }
}
