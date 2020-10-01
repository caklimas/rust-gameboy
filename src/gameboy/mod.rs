use serde::{Serialize, Deserialize};
use crate::cartridge::Cartridge;
use crate::cpu;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize)]
pub struct Gameboy {
    cpu: cpu::Cpu
}

impl Gameboy {
    pub fn new(bytes: Vec<u8>) -> Self {
        Gameboy {
            cpu: cpu::Cpu::new(Cartridge::new(bytes))
        }
    }

    pub fn run(&mut self) {
        loop {
            self.cpu.clock();
        }
    }
}