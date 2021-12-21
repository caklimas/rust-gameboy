use super::super::high_ram::HighRam;
use super::super::memory_sizes::HIGH_RAM;

#[test]
fn read_test() {
    let address = 5;
    let data = 5;
    let mut high_ram: HighRam = Default::default();
    high_ram.write(address, data);

    let actual = high_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let address = HIGH_RAM + address;
    let actual = high_ram.read(address);
    assert_eq!(data, actual);
}

#[test]
fn write_test() {
    let address = 5;
    let data = 5;
    let mut high_ram: HighRam = Default::default();
    high_ram.write(address, data);

    let actual = high_ram.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let data = data + 1;
    let address = HIGH_RAM + address;
    high_ram.write(address, data);
    let actual = high_ram.read(address);
    assert_eq!(data, actual);
}
