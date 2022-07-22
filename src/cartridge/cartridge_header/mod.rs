pub mod cartridge_type;
pub mod ram_size;
pub mod rom_size;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

pub const CARTRIDGE_HEADER_INDEX: usize = 0x147;
pub const CHECKSUM_INDEX: usize = 0x14D;
pub const CHECKSUM_RANGE: std::ops::RangeInclusive<usize> = 0x134..=0x14C;
pub const LOGO_INDEX_LOWER: usize = 0x104;
pub const LOGO_INDEX_UPPER: usize = 0x133;
pub const NAME_INDEX_LOWER: usize = 0x134;
pub const NAME_INDEX_UPPER: usize = 0x142;
pub const RAM_SIZE_INDEX: usize = 0x149;
pub const ROM_SIZE_INDEX: usize = 0x148;
pub const SGB_INDEX: usize = 0x146;

const LOGO_SIZE: usize = 48;
const NAME_SIZE: usize = 15;

#[derive(Serialize, Deserialize)]
pub struct CartridgeHeader {
    pub cartridge_type: cartridge_type::CartridgeType,
    pub ram_size: ram_size::RamSize,
    pub rom_size: rom_size::RomSize,
    #[serde(with = "BigArray")]
    logo: [u8; LOGO_SIZE],
    name: [u8; NAME_SIZE],
    sgb_flag: u8,
}

impl CartridgeHeader {
    pub fn new(bytes: &[u8], check_checksum: bool) -> Self {
        let mut logo: [u8; LOGO_SIZE] = [0; LOGO_SIZE];
        logo.copy_from_slice(&bytes[LOGO_INDEX_LOWER..=LOGO_INDEX_UPPER]);

        let mut name: [u8; NAME_SIZE] = [0; NAME_SIZE];
        name.copy_from_slice(&bytes[NAME_INDEX_LOWER..=NAME_INDEX_UPPER]);

        if check_checksum && !CartridgeHeader::valid_checksum(bytes) {
            panic!("Invalid checksum");
        }

        CartridgeHeader {
            cartridge_type: cartridge_type::CartridgeType::new(&bytes[CARTRIDGE_HEADER_INDEX]),
            ram_size: ram_size::RamSize::new(&bytes[RAM_SIZE_INDEX]),
            rom_size: rom_size::RomSize::new(&bytes[ROM_SIZE_INDEX]),
            logo,
            name,
            sgb_flag: bytes[SGB_INDEX],
        }
    }

    fn valid_checksum(bytes: &[u8]) -> bool {
        let checksum = bytes[CHECKSUM_INDEX];
        let mut value: u8 = 0;
        for i in CHECKSUM_RANGE {
            value = value.wrapping_sub(bytes[i]).wrapping_sub(1);
        }

        checksum == value
    }
}

impl Default for CartridgeHeader {
    fn default() -> Self {
        Self {
            logo: [0; LOGO_SIZE],
            cartridge_type: Default::default(),
            ram_size: Default::default(),
            rom_size: Default::default(),
            name: Default::default(),
            sgb_flag: Default::default(),
        }
    }
}
