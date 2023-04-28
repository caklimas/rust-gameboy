use crate::cartridge::cartridge_header::cgb_mode::CgbMode;

#[test]
fn returns_correct_cgb_mode() {
    let cgb_mode = CgbMode::new(&0x80);
    assert_eq!(CgbMode::CgbMonochrome, cgb_mode);

    let cgb_mode = CgbMode::new(&0xC0);
    assert_eq!(CgbMode::CgbOnly, cgb_mode);

    let cgb_mode = CgbMode::new(&0);
    assert_eq!(CgbMode::NonCgb, cgb_mode);
}
