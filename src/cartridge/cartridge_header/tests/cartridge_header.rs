use super::super::cartridge_type::CartridgeType;
use super::super::ram_size::RamSize;
use super::super::rom_size::RomSize;
use super::super::CartridgeHeader;

#[test]
fn new_cartridge_type_test() {
    let cartridge_type_value = 1;
    let mut bytes = [0; 0x200];
    bytes[super::super::CARTRIDGE_HEADER_INDEX] = cartridge_type_value;

    let header = CartridgeHeader::new(&bytes, false);

    assert_eq!(CartridgeType::Mbc1, header.cartridge_type);
}

#[test]
fn new_logo_test() {
    let offset = 1;
    let logo_value = 1;
    let mut bytes = [0; 0x200];
    bytes[super::super::LOGO_INDEX_LOWER + offset] = logo_value;

    let header = CartridgeHeader::new(&bytes, false);

    assert_eq!(logo_value, header.logo[offset]);
}

#[test]
fn new_name_test() {
    let offset = 1;
    let name_value = 1;
    let mut bytes = [0; 0x200];
    bytes[super::super::NAME_INDEX_LOWER + offset] = name_value;

    let header = CartridgeHeader::new(&bytes, false);

    assert_eq!(name_value, header.name[offset]);
}

#[test]
fn new_ram_size_test() {
    // Kilobytes_2
    let ram_size_value = 1;
    let mut bytes = [0; 0x200];
    bytes[super::super::RAM_SIZE_INDEX] = ram_size_value;

    let header = CartridgeHeader::new(&bytes, false);

    assert_eq!(RamSize::Kilobytes_2, header.ram_size);
}

#[test]
fn new_rom_size_test() {
    // Kilobytes_2
    let rom_size_value = 1;
    let mut bytes = [0; 0x200];
    bytes[super::super::ROM_SIZE_INDEX] = rom_size_value;

    let header = CartridgeHeader::new(&bytes, false);

    assert_eq!(RomSize::Kilobyte_64, header.rom_size);
}

#[test]
fn new_sgb_flag_test() {
    let sgb_value = 1;
    let mut bytes = [0; 0x200];
    bytes[super::super::SGB_INDEX] = sgb_value;

    let header = CartridgeHeader::new(&bytes, false);

    assert_eq!(sgb_value, header.sgb_flag);
}
