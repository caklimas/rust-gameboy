use serde::{Serialize, Deserialize};
use super::memory_sizes::{HIGH_RAM};

#[derive(Serialize, Deserialize)]
pub struct HighRam {
    data: Vec<u8>
}

impl HighRam {
    pub fn read(&self, address: u16) -> u8 {
        let masked_address = self.get_masked_address(address); 
        self.data[masked_address]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let masked_address = self.get_masked_address(address);
        self.data[masked_address] = data;
    }

    fn get_masked_address(&self, address: u16) -> usize {
        (address % HIGH_RAM) as usize
    }
}

impl Default for HighRam {
    fn default() -> Self {
        HighRam {
            data: vec![0; HIGH_RAM as usize]
        }
    }
}