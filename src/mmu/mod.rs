pub mod boot_rom;
pub mod high_ram;
pub mod interrupts;
pub mod memory_sizes;
pub mod ram;
pub mod serial_data_transfer;
pub mod timer;
pub mod work_ram;

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use crate::addresses::boot_rom::*;
use crate::addresses::cartridge::*;
use crate::addresses::controls::*;
use crate::cartridge::Cartridge;
use crate::constants::boot_rom::*;
use crate::controls::*;
use crate::input::*;

#[derive(Serialize, Deserialize, Default)]
pub struct Mmu {
    pub ram: ram::Ram,
    boot_rom: boot_rom::BootRom,
    boot_rom_finished: bool,
    cartridge: Option<Cartridge>,
    controls: Controls,
    previous_input: Option<Input>,
    run_boot_rom: bool
}

impl Mmu {
    pub fn new(cartridge: Cartridge, run_boot_rom: bool) -> Self {
        let mut mmu = Mmu {
            ram: Default::default(),
            boot_rom: Default::default(),
            boot_rom_finished: !run_boot_rom,
            cartridge: Some(cartridge),
            controls: Default::default(),
            previous_input: None,
            run_boot_rom
        };

        if !mmu.run_boot_rom {
            mmu.program_start();
        }

        mmu
    }

    pub fn clock(&mut self, cycles: u16, input: &Input) {
        if let Some(i) = self.previous_input {
            let copy = (*input).clone();
            if copy != i {
                self.controls.update_input(input);
                self.previous_input = Some(copy);
            }
        }

        self.ram.interrupt_flag.set_joypad(self.controls.interrupt);
        self.ram.clock(cycles)
    }

    pub fn finish_running_boot_rom(&mut self) {
        if !self.boot_rom_finished {
            panic!("Boot rom didn't finish correctly");
        }

        self.run_boot_rom = false;
    }
    
    pub fn read_word(&self, address: u16) -> u16 {
        let low = self.read_byte(address) as u16;
        let high = self.read_byte(address + 1) as u16;

        (high << 8) | low
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match (address, self.run_boot_rom) {
            (BOOT_ROM_LOWER..=BOOT_ROM_UPPER, true) => self.boot_rom.read(address),
            (CART_ROM_LOWER..=CART_ROM_UPPER, _) => self.read_mbc_rom(address),
            (CART_EXTERNAL_RAM_LOWER..=CART_EXTERNAL_RAM_UPPER, _) => self.read_mbc_ram(address),
            (CONTROLS, _) => self.controls.read_byte(),
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
        match (address, self.run_boot_rom) {
            (BOOT_ROM_LOWER..=BOOT_ROM_UPPER, true) => self.boot_rom.write(address, data),
            (CART_ROM_LOWER..=CART_ROM_UPPER, _) => self.write_mbc_rom(address, data),
            (CART_EXTERNAL_RAM_LOWER..=CART_EXTERNAL_RAM_UPPER, _) => self.write_mbc_ram(address, data),
            (BOOT_ROM_FINISHED, _) => self.boot_rom_finished = data == BOOT_ROM_FINISHED_VALUE,
            (CONTROLS, _) => self.controls.write_byte(data),
            _ => self.ram.write(address, data)
        }
    }

    fn program_start(&mut self) {
        self.write_byte(0xFF05, 0x00);
        self.write_byte(0xFF06, 0x00);
        self.write_byte(0xFF07, 0x00);
        self.write_byte(0xFF10, 0x80);
        self.write_byte(0xFF11, 0xBF);
        self.write_byte(0xFF12, 0xF3);
        self.write_byte(0xFF14, 0xBF);
        self.write_byte(0xFF16, 0x3F);
        self.write_byte(0xFF17, 0x00);
        self.write_byte(0xFF19, 0xBF);
        self.write_byte(0xFF1A, 0x7F);
        self.write_byte(0xFF1B, 0xFF);
        self.write_byte(0xFF1C, 0x9F);
        self.write_byte(0xFF1E, 0xBF);
        self.write_byte(0xFF20, 0xFF);
        self.write_byte(0xFF21, 0x00);
        self.write_byte(0xFF22, 0x00);
        self.write_byte(0xFF23, 0xBF);
        self.write_byte(0xFF24, 0x77);
        self.write_byte(0xFF25, 0xF3);
        self.write_byte(0xFF26, 0xF1);
        self.write_byte(0xFF40, 0x91);
        self.write_byte(0xFF42, 0x00);
        self.write_byte(0xFF43, 0x00);
        self.write_byte(0xFF45, 0x00);
        self.write_byte(0xFF47, 0xFC);
        self.write_byte(0xFF48, 0xFF);
        self.write_byte(0xFF49, 0xFF);
        self.write_byte(0xFF4A, 0x00);
        self.write_byte(0xFF4B, 0x00);
        self.write_byte(0xFFFF, 0x00);
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