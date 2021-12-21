use super::super::cartridge_type::CartridgeType;

#[test]
fn new_test() {
    let value = 0x01;
    let ct = CartridgeType::new(&value);

    assert_eq!(CartridgeType::Mbc1, ct);
}

#[test]
#[should_panic]
fn new_panic_test() {
    let value = 0x30;
    CartridgeType::new(&value);
}