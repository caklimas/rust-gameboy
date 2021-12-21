use super::super::banking_mode::BankingMode;
use super::super::{Mbc, Mbc1, ENABLE_RAM};
use crate::addresses::mbc::mbc1::*;
use crate::cartridge::cartridge_header::{CartridgeHeader, RAM_SIZE_INDEX};
use crate::mmu::memory_sizes::{KILOBYTES_16, KILOBYTES_8};

#[test]
fn read_ram_test() {
    let data = 5;
    let mut bytes = vec![0; 0x200];
    bytes[RAM_SIZE_INDEX] = 1;
    let header = CartridgeHeader::new(&bytes, false);
    let mut mbc = Mbc1::new(&header, bytes);
    mbc.ram[0] = data;

    let result = mbc.read_ram(0);

    assert_ne!(data, result);

    mbc.ram_enabled = true;
    mbc.ram[0] = data;

    let result = mbc.read_ram(KILOBYTES_8);

    assert_eq!(data, result);
}

#[test]
fn read_rom_test() {
    let data = 5;
    let mut bytes = vec![0; (KILOBYTES_16 * 2) as usize];
    bytes[0] = data;
    bytes[KILOBYTES_16 as usize] = data;
    let header = CartridgeHeader::new(&bytes, false);
    let mbc = Mbc1::new(&header, bytes);

    let result = mbc.read_rom(0);

    assert_eq!(data, result);
    let result = mbc.read_rom(KILOBYTES_16 * 2);

    assert_eq!(data, result);
}

#[test]
fn write_bank_mode_test() {
    let bytes = vec![0; 0x200];
    let header = CartridgeHeader::new(&bytes, false);
    let mut mbc = Mbc1::new(&header, bytes);

    mbc.write_rom(BANKING_MODE_SELECT_LOWER, 0);

    assert_eq!(BankingMode::Rom, mbc.bank_mode);

    mbc.write_rom(BANKING_MODE_SELECT_LOWER, 1);

    assert_eq!(BankingMode::Ram, mbc.bank_mode);
}

#[test]
fn write_bank_number_ram_test() {
    let data = 0b0010;
    let bytes = vec![0; 0x200];
    let header = CartridgeHeader::new(&bytes, false);
    let mut mbc = Mbc1::new(&header, bytes);
    mbc.bank_mode = BankingMode::Ram;

    mbc.write_rom(RAM_BANK_NUMBER_LOWER, data);

    assert_eq!(data, mbc.ram_bank_number);
}

#[test]
fn write_bank_number_rom_test() {
    let data = 0b0010;
    let bytes = vec![0; 0x200];
    let header = CartridgeHeader::new(&bytes, false);
    let mut mbc = Mbc1::new(&header, bytes);
    mbc.rom_bank_number = 0;
    mbc.bank_mode = BankingMode::Rom;

    mbc.write_rom(RAM_BANK_NUMBER_LOWER, data);

    assert_eq!(data << 5, mbc.rom_bank_number);
}

#[test]
fn write_ram_enabled_test() {
    let bytes = vec![0; 0x200];
    let header = CartridgeHeader::new(&bytes, false);
    let mut mbc = Mbc1::new(&header, bytes);

    mbc.write_rom(RAM_ENABLE_LOWER, ENABLE_RAM);

    assert_eq!(true, mbc.ram_enabled);

    mbc.write_rom(RAM_ENABLE_LOWER, ENABLE_RAM - 1);

    assert_eq!(false, mbc.ram_enabled);
}

#[test]
fn write_rom_bank_number_lower_test() {
    let data = 0b0101;
    let bytes = vec![0; 0x200];
    let header = CartridgeHeader::new(&bytes, false);
    let mut mbc = Mbc1::new(&header, bytes);

    mbc.write_rom(ROM_BANK_NUMBER_LOWER, data);

    assert_eq!(data, mbc.rom_bank_number);

    mbc.write_rom(ROM_BANK_NUMBER_LOWER, 0);

    assert_eq!(1, mbc.rom_bank_number);
}
