use serde::{Deserialize, Serialize};

use self::mbc0::Mbc0;
use self::mbc1::Mbc1;
use self::mbc3::Mbc3;

use super::cartridge::cartridge_header::cartridge_type::CartridgeType;
use super::cartridge::cartridge_header::CartridgeHeader;

pub mod mbc0;
pub mod mbc1;
pub mod mbc3;

pub fn get_mbc(header: &CartridgeHeader, data: Vec<u8>) -> Mbc {
    match header.cartridge_type {
        CartridgeType::RomOnly => Mbc::new(MbcType::Mbc0(Mbc0::new(data))),
        CartridgeType::Mbc1 | CartridgeType::Mbc1Ram | CartridgeType::Mbc1RamBattery => {
            Mbc::new(MbcType::Mbc1(Mbc1::new(header, data)))
        }
        CartridgeType::Mbc3
        | CartridgeType::Mbc3Ram
        | CartridgeType::Mbc3RamBattery
        | CartridgeType::Mbc3TimerBattery
        | CartridgeType::Mbc3TimerRamBattery => Mbc::new(MbcType::Mbc3(Mbc3::new(header, data))),
        _ => panic!("Unsupported mbc: {:?}", header.cartridge_type),
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Mbc {
    mbc_type: MbcType,
}

impl Mbc {
    pub fn new(mbc_type: MbcType) -> Self {
        Self { mbc_type }
    }

    pub fn read_ram(&self, address: u16) -> u8 {
        match &self.mbc_type {
            MbcType::Mbc0(mbc) => mbc.read_ram(address),
            MbcType::Mbc1(mbc) => mbc.read_ram(address),
            MbcType::Mbc3(mbc) => mbc.read_ram(address),
            MbcType::Unknown => 0,
        }
    }

    pub fn read_rom(&self, address: u16) -> u8 {
        match &self.mbc_type {
            MbcType::Mbc0(mbc) => mbc.read_rom(address),
            MbcType::Mbc1(mbc) => mbc.read_rom(address),
            MbcType::Mbc3(mbc) => mbc.read_rom(address),
            MbcType::Unknown => 0,
        }
    }

    pub fn write_ram(&mut self, address: u16, value: u8) {
        match &mut self.mbc_type {
            MbcType::Mbc0(mbc) => mbc.write_ram(address, value),
            MbcType::Mbc1(mbc) => mbc.write_ram(address, value),
            MbcType::Mbc3(mbc) => mbc.write_ram(address, value),
            MbcType::Unknown => (),
        }
    }

    pub fn write_rom(&mut self, address: u16, value: u8) {
        match &mut self.mbc_type {
            MbcType::Mbc0(mbc) => mbc.write_rom(address, value),
            MbcType::Mbc1(mbc) => mbc.write_rom(address, value),
            MbcType::Mbc3(mbc) => mbc.write_rom(address, value),
            MbcType::Unknown => (),
        }
    }

    pub fn set_ram(&mut self, data: Vec<u8>) {
        match &mut self.mbc_type {
            MbcType::Unknown | MbcType::Mbc0(_) => (),
            MbcType::Mbc1(mbc) => mbc.set_ram(data),
            MbcType::Mbc3(mbc) => mbc.set_ram(data),
        }
    }

    pub fn has_battery(&self) -> bool {
        match &self.mbc_type {
            MbcType::Unknown => false,
            MbcType::Mbc0(mbc) => mbc.has_battery(),
            MbcType::Mbc1(mbc) => mbc.has_battery(),
            MbcType::Mbc3(mbc) => mbc.has_battery(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum MbcType {
    Unknown,
    Mbc0(Mbc0),
    Mbc1(Mbc1),
    Mbc3(Mbc3),
}

impl Default for MbcType {
    fn default() -> Self {
        MbcType::Unknown
    }
}
