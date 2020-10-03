use serde::{Serialize, Deserialize};
use crate::cartridge::Cartridge;
use crate::constants::cpu::*;
use crate::cpu;

#[cfg(test)]
mod tests;

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

    pub fn run(&mut self) {
        let wait_ticks = ((CPU_REFRESH_RATE as f64) / 1000.0 * 16.0).round() as u32;
        let mut ticks = 0;
        loop {
            while ticks < wait_ticks {
                ticks += self.cpu.clock() as u32;
            }
    
            ticks -= wait_ticks;
        }
    }
}