use super::super::rom_size::RomSize;

#[test]
fn new_test() {
    let value = 0x01;
    let rs = RomSize::new(&value);

    assert_eq!(RomSize::Kilobyte_64, rs);
}

#[test]
#[should_panic]
fn new_panic_test() {
    let value = 0x30;
    RomSize::new(&value);
}
