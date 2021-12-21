use crate::mmu::memory_sizes::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum RamSize {
    None,
    Kilobytes_2,
    Kilobytes_8,
    Kilibytes_32,  // (4 banks of 8KBytes each)
    Kilobytes_128, // (16 banks of 8KBytes each)
    Kilobytes_64,  // (8 banks of 8KBytes each)
}

impl RamSize {
    pub fn new(value: &u8) -> Self {
        match value {
            0x00 => RamSize::None,
            0x01 => RamSize::Kilobytes_2,
            0x02 => RamSize::Kilobytes_8,
            0x03 => RamSize::Kilibytes_32,
            0x04 => RamSize::Kilobytes_128,
            0x05 => RamSize::Kilobytes_64,
            _ => panic!("Invalid ram size 0x{:2X}", value),
        }
    }

    pub fn get_size(&self) -> usize {
        match self {
            RamSize::None => 0,
            RamSize::Kilobytes_2 => KILOBYTES_2 as usize,
            RamSize::Kilobytes_8 => KILOBYTES_8 as usize,
            RamSize::Kilibytes_32 => KILOBYTES_32 as usize,
            RamSize::Kilobytes_64 => KILOBYTES_64,
            RamSize::Kilobytes_128 => KILOBYTES_128,
        }
    }
}
