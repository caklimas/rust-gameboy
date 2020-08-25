use super::super::memory_sizes::KILOBYTES_8;
use super::super::work_ram::WorkRam;

#[test]
fn read_test() {
    let address = 5;
    let data = 5;
    let mut work_ram: WorkRam = Default::default();
    work_ram.write(address, data);

    let actual = work_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let address = KILOBYTES_8 + address;
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