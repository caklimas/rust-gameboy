pub mod render;

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};
use crate::cartridge::Cartridge;
use crate::cpu;
use crate::input::*;

#[derive(Serialize, Deserialize)]
pub struct Gameboy {
    cpu: cpu::Cpu
}

impl Gameboy {
    pub fn new(bytes: Vec<u8>, run_boot_rom: bool) -> Self {
        Gameboy {
            cpu: cpu::Cpu::new(Cartridge::new(bytes), run_boot_rom)
        }
    }

    pub fn clock(&mut self, input: &Input) -> u16 {
        self.cpu.clock(input)
    }

    pub fn frame_complete(&mut self) -> bool {
        self.cpu.frame_complete()
    }

    pub fn get_cycles(&mut self) -> u32 {
        self.cpu.master_clock_cycles
    }

    pub fn get_screen(&mut self) -> &[u8] {
        self.cpu.get_screen()
    }
}