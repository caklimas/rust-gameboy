pub mod cartridge_header;

use serde::{Serialize, Deserialize};
use super::mbc::{Mbc, get_mbc};

#[derive(Serialize, Deserialize)]
pub struct Cartridge {
    #[serde(skip)]
    pub mbc: Option<Box<dyn Mbc>>,
    header: cartridge_header::CartridgeHeader
}

impl Cartridge {
    pub fn new(bytes: Vec<u8>) -> Self {
        let header = cartridge_header::CartridgeHeader::new(&bytes, true);
        let mbc = get_mbc(&header, bytes); 

        Cartridge {
            header, 
            mbc
        }
    }
}