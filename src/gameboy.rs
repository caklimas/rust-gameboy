use serde::{Serialize, Deserialize};
use crate::cpu;

#[derive(Serialize, Deserialize)]
pub struct Gameboy {
    cpu: cpu::Cpu
}

impl Gameboy {
    pub fn new(file_path: &str) -> Self {
        Gameboy {
            cpu: Default::default()
        }
    }
}