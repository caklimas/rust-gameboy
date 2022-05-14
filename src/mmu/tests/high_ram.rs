use crate::addresses::high_ram::HIGH_RAM_LOWER;

use super::super::high_ram::HighRam;

#[test]
fn read_test() {
    let address = HIGH_RAM_LOWER + 5;
    let data = 5;
    let mut high_ram: HighRam = Default::default();
    high_ram.write(address, data);

    let actual = high_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = HIGH_RAM_LOWER + 5;
    let data = 5;
    let mut high_ram: HighRam = Default::default();
    high_ram.write(address, data);

    let actual = high_ram.read(address);
    assert_eq!(data, actual);
}
