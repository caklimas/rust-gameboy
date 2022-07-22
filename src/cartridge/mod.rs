pub mod cartridge_header;

use super::mbc::{get_mbc, Mbc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Cartridge {
    pub mbc: Mbc,
    header: cartridge_header::CartridgeHeader,
}

impl Cartridge {
    pub fn new(bytes: Vec<u8>) -> Self {
        let header = cartridge_header::CartridgeHeader::new(&bytes, true);
        let mbc = get_mbc(&header, bytes);

        Cartridge { header, mbc }
    }

    pub fn from_save_data(bytes: Vec<u8>, save_data: Vec<u8>) -> Self {
        let mut c = Cartridge::new(bytes);
        c.mbc.set_ram(save_data);
        c
    }
}
