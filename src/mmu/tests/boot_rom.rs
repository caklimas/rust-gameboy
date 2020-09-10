use super::super::memory_sizes::BOOT_ROM_SIZE;
use super::super::boot_rom::BootRom;

#[test]
fn read_test() {
    let address = 0;
    let data = 0x31;
    let boot_rom: BootRom = Default::default();
    let actual = boot_rom.read(address);
    assert_eq!(data, actual);

    // Testing masking
    let address = (BOOT_ROM_SIZE as u16) + address;
    let actual = boot_rom.read(address);
    assert_eq!(data, actual);
}

#[test]
#[should_panic]
fn write_test() {
    let mut boot_rom: BootRom = Default::default();
    boot_rom.write(0, 0);
}