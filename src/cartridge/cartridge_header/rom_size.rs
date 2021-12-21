use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum RomSize {
    Kilobyte_32,
    Kilobyte_64,
    Kilobyte_128,
    Kilobyte_256,
    Kilobyte_512,
    Megabyte_1,
    Megabyte_2,
    Megabyte_4,
    Megabyte_8,
    Megabyte_1_1,
    Megabyte_1_2,
    Megabyte_1_5
}

impl RomSize {
    pub fn new(value: &u8) -> Self {
        match value {
            0x00 => RomSize::Kilobyte_32,
            0x01 => RomSize::Kilobyte_64,
            0x02 => RomSize::Kilobyte_128,
            0x03 => RomSize::Kilobyte_256,
            0x04 => RomSize::Kilobyte_512,
            0x05 => RomSize::Megabyte_1,
            0x06 => RomSize::Megabyte_2,
            0x07 => RomSize::Megabyte_4,
            0x08 => RomSize::Megabyte_8,
            0x52 => RomSize::Megabyte_1_1,
            0x53 => RomSize::Megabyte_1_2,
            0x54 => RomSize::Megabyte_1_5,
            _ => panic!("Invalid rom size 0x{:2X}", value)
        }
    }
}