use super::cartridge::cartridge_header::CartridgeHeader;
use super::cartridge::cartridge_header::cartridge_type::CartridgeType;

pub mod mbc0;
pub mod mbc1;

pub trait Mbc {
    fn read_ram(&self, address: u16) -> u8;
    fn read_rom(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, data: u8);
    fn write_rom(&mut self, address: u16, data: u8);
}

pub fn get_mbc(header: &CartridgeHeader, data: Vec<u8>) -> Option<Box<dyn Mbc>> {
    match header.cartridge_type {
        CartridgeType::RomOnly => Some(Box::new(mbc0::Mbc0::new(data))),
        CartridgeType::Mbc1 | CartridgeType::Mbc1Ram | CartridgeType::Mbc1RamBattery => Some(Box::new(mbc1::Mbc1::new(header, data))),
        _ => panic!("Unsupported mbc: {:?}", header.cartridge_type)
    }
}