use super::super::memory_sizes::KILOBYTES_8;
use super::super::work_ram::WorkRam;
use crate::addresses::work_ram::WORK_RAM_ECHO_LOWER;

#[test]
fn read_test() {
    let address_offset = 5;
    let data = 5;
    let mut work_ram: WorkRam = Default::default();
    work_ram.write(address_offset, data);

    let actual = work_ram.read(address_offset);
    assert_eq!(data, actual);

    // Testing masking
    let address = KILOBYTES_8 + address_offset;
    let actual = work_ram.read(address);
    assert_eq!(data, actual);

    let address = WORK_RAM_ECHO_LOWER + address_offset;
    let actual = work_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = 5;
    let data = 5;
    let mut work_ram: WorkRam = Default::default();
    work_ram.write(address, data);

    let actual = work_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let data = data + 1;
    let address = KILOBYTES_8 + address;
    work_ram.write(address, data);
    let actual = work_ram.read(address);
    assert_eq!(data, actual);
}
