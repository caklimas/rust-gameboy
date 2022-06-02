pub mod banking_mode;
pub mod rtc;

use crate::{
    addresses::mbc::mbc3::{
        LATCH_CLOCK_LOWER, LATCH_CLOCK_UPPER, RAM_BANK_NUMBER_RTC_LOWER, RAM_BANK_NUMBER_RTC_UPPER,
        RAM_ENABLE_LOWER, RAM_ENABLE_UPPER, ROM_BANK_0_LOWER, ROM_BANK_0_UPPER,
        ROM_BANK_1_7F_LOWER, ROM_BANK_1_7F_UPPER, ROM_BANK_NUMBER_LOWER, ROM_BANK_NUMBER_UPPER,
        RTC_REGISTER_LOWER, RTC_REGISTER_UPPER,
    },
    cartridge::cartridge_header::CartridgeHeader,
    mmu::memory_sizes::KILOBYTES_8,
};

use self::{banking_mode::BankingMode, rtc::Rtc};

use super::Mbc;

const ENABLE_RAM: u8 = 0x0A;

pub struct Mbc3 {
    ram: Vec<u8>,
    ram_bank_number: u8,
    ram_enabled: bool,
    rom: Vec<u8>,
    rom_bank_number: u8,
    bank_mode: BankingMode,
    rtc_register: u8,
    latch_state: bool,
    rtc: Rtc,
}

impl Mbc3 {
    pub fn new(header: &CartridgeHeader, data: Vec<u8>) -> Self {
        Self {
            ram: vec![0xFF; header.ram_size.get_size()],
            ram_bank_number: 0x00,
            ram_enabled: false,
            rom: data,
            rom_bank_number: 0x01,
            bank_mode: BankingMode::Ram,
            rtc_register: 0,
            latch_state: false,
            rtc: Default::default(),
        }
    }

    fn write_ram_enabled(&mut self, data: u8) {
        self.ram_enabled = data == ENABLE_RAM;
    }

    fn write_rom_bank_number(&mut self, data: u8) {
        self.rom_bank_number = match data & 0b111_1111 {
            0 => 1,
            d => d,
        };
    }

    fn write_ram_bank(&mut self, data: u8) {
        match data {
            0x00..=0x03 => {
                self.bank_mode = BankingMode::Ram;
                self.ram_bank_number = data;
            }
            0x08..=0x0C => {
                self.bank_mode = BankingMode::Rtc;
                self.rtc_register = data;
            }
            _ => (),
        }
    }

    fn write_latch_data(&mut self, data: u8) {
        match data & 0x01 {
            0x01 if self.latch_state => {
                self.latch_state = false;
                self.rtc.calculate_clock();
            }
            0x00 if !self.latch_state => self.latch_state = true,
            _ => self.latch_state = false,
        }
    }

    fn write_ram(&mut self, address: u16, data: u8) {
        let index = self.get_ram_index(address);
        self.ram[index] = data;
    }

    fn write_rtc_register(&mut self, data: u8) {
        self.rtc.write(self.rtc_register, data);
    }

    fn get_ram_index(&self, address: u16) -> usize {
        self.ram_bank_number as usize * (KILOBYTES_8 as usize)
            + (address as usize - RTC_REGISTER_LOWER as usize)
    }
}

impl Mbc for Mbc3 {
    fn read_ram(&self, address: u16) -> u8 {
        if !self.ram_enabled {
            return 0;
        }

        match self.bank_mode {
            BankingMode::Ram => self.ram[self.get_ram_index(address)],
            BankingMode::Rtc => self.rtc.read(self.rtc_register),
        }
    }

    fn read_rom(&self, address: u16) -> u8 {
        match address {
            ROM_BANK_0_LOWER..=ROM_BANK_0_UPPER => self.rom[address as usize],
            ROM_BANK_1_7F_LOWER..=ROM_BANK_1_7F_UPPER => {
                let index = self.rom_bank_number as usize * ROM_BANK_1_7F_LOWER as usize
                    + (address as usize - ROM_BANK_1_7F_LOWER as usize);
                self.rom[index as usize]
            }
            _ => panic!("Invalid MBC3 address 0x{:4X}", address),
        }
    }

    fn write_ram(&mut self, address: u16, data: u8) {
        if !self.ram_enabled {
            return;
        }

        if let RTC_REGISTER_LOWER..=RTC_REGISTER_UPPER = address {
            match self.bank_mode {
                BankingMode::Ram => self.write_ram(address, data),
                BankingMode::Rtc => self.write_rtc_register(data),
            }
        }
    }

    fn write_rom(&mut self, address: u16, data: u8) {
        match address {
            RAM_ENABLE_LOWER..=RAM_ENABLE_UPPER => self.write_ram_enabled(data),
            ROM_BANK_NUMBER_LOWER..=ROM_BANK_NUMBER_UPPER => self.write_rom_bank_number(data),
            RAM_BANK_NUMBER_RTC_LOWER..=RAM_BANK_NUMBER_RTC_UPPER => self.write_ram_bank(data),
            LATCH_CLOCK_LOWER..=LATCH_CLOCK_UPPER => self.write_latch_data(data),
            _ => (),
        }
    }
}
