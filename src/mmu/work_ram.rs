use serde::{Serialize, Deserialize};
use super::memory_sizes::{KILOBYTES_8};
use crate::addresses::work_ram::WORK_RAM_ECHO_LOWER;

big_array! { BigArray; }

#[derive(Serialize, Deserialize)]
pub struct WorkRam {
    #[serde(with = "BigArray")]
    data: [u8; KILOBYTES_8 as usize]
}

impl WorkRam {
    pub fn read(&self, address: u16) -> u8 {
        let masked_address = self.get_masked_address(address); 
        self.data[masked_address]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let masked_address = self.get_masked_address(address);
        self.data[masked_address] = data;
    }

    fn get_masked_address(&self, address: u16) -> usize {
        let masked_address = if address >= WORK_RAM_ECHO_LOWER {
            address - 0x2000
        } else {
            address
        };

        (masked_address % KILOBYTES_8) as usize
    }
}

impl Default for WorkRam {
    fn default() -> Self {
        WorkRam {
            data: [0; KILOBYTES_8 as usize]
        }
    }
}