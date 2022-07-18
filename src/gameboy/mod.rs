pub mod render;

#[cfg(test)]
mod tests;

use crate::cartridge::Cartridge;
use crate::cpu;
use crate::input::Input;
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize)]
pub struct Gameboy {
    cpu: cpu::Cpu,
    pub input: Input,
}

impl Gameboy {
    pub fn new(bytes: Vec<u8>, run_boot_rom: bool) -> Self {
        Gameboy {
            cpu: cpu::Cpu::new(Cartridge::new(bytes), run_boot_rom),
            input: Input::new(),
        }
    }

    pub fn clock(&mut self) -> (u16, bool) {
        self.cpu.clock()
    }

    pub fn frame_complete(&mut self) -> bool {
        self.cpu.frame_complete()
    }

    pub fn get_cycles(&mut self) -> u32 {
        self.cpu.master_clock_cycles
    }

    pub fn get_controls(&self) -> Input {
        self.input
    }

    pub fn get_screen(&self) -> &[u8] {
        self.cpu.get_screen()
    }

    pub fn get_audio_buffer(&self) -> &[f32] {
        self.cpu.get_audio_buffer()
    }

    pub fn update_controls(&mut self, input: Input) {
        self.input = input;
        self.cpu.mmu.update_controls(input);
    }
}
