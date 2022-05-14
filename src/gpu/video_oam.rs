use crate::{addresses::gpu::sprite::SPRITE_ATTRIBUTE_TABLE_LOWER, mmu::memory_sizes::*};
use serde::{Deserialize, Serialize};

big_array! { BigArray; }

#[derive(Serialize, Deserialize)]
pub struct VideoOam {
    #[serde(with = "BigArray")]
    data: [u8; VIDEO_OAM],
}

impl VideoOam {
    pub fn read(&self, address: u16) -> u8 {
        let masked_address = self.get_masked_address(address);
        self.data[masked_address]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let masked_address = self.get_masked_address(address);
        self.data[masked_address] = data;
    }

    fn get_masked_address(&self, address: u16) -> usize {
        (address - SPRITE_ATTRIBUTE_TABLE_LOWER) as usize
    }
}

impl Default for VideoOam {
    fn default() -> Self {
        VideoOam {
            data: [0; VIDEO_OAM as usize],
        }
    }
}
