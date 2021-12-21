use crate::mmu::memory_sizes::{KILOBYTES_32};
use crate::cartridge::cartridge_header::{RAM_SIZE_INDEX};
use super::super::Mbc;
use super::super::mbc0::Mbc0;

#[test]
fn read_ram_test() {
    let data = 5;
    let mut bytes = vec![0; 0x200];
    bytes[RAM_SIZE_INDEX] = 1;
    let mut mbc = Mbc0::new(bytes);
    mbc.ram[0] = data;

    let result = mbc.read_ram(0);

    assert_eq!(data, result);
}

#[test]
fn read_rom_test() {
    let data = 5;
    let mut bytes = vec![0; (KILOBYTES_32) as usize];
    bytes[0] = data;
    let mbc = Mbc0::new(bytes);

    let result = mbc.read_rom(0);

    assert_eq!(data, result);
    let result = mbc.read_rom(KILOBYTES_32);

    assert_eq!(data, result);
}

#[test]
fn write_ram_test() {
    let data = 5;
    let mut bytes = vec![0; 0x200];
    bytes[RAM_SIZE_INDEX] = 1;
    let mut mbc = Mbc0::new(bytes);

    mbc.write_ram(0, data);

    assert_eq!(data, mbc.ram[0]);
}