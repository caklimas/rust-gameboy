use super::super::ram_size::RamSize;

#[test]
fn new_test() {
    let value = 0x01;
    let rs = RamSize::new(&value);

    assert_eq!(RamSize::Kilobytes_2, rs);
}

#[test]
#[should_panic]
fn new_panic_test() {
    let value = 0x30;
    RamSize::new(&value);
}