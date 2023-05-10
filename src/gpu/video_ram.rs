use std::slice::Chunks;

use crate::{addresses::gpu::video_ram::VIDEO_RAM_LOWER, mmu::memory_sizes::KILOBYTES_8};
use serde::{Deserialize, Serialize};

use serde_big_array::BigArray;

#[derive(Serialize, Deserialize)]
pub struct VideoRam {
    #[serde(with = "BigArray")]
    data: [u8; KILOBYTES_8 as usize],
}

impl VideoRam {
    pub fn read(&self, address: u16) -> u8 {
        let masked_address = self.get_masked_address(address);
        self.data[masked_address]
    }

    pub fn write(&mut self, address: u16, data: u8) {
        let masked_address = self.get_masked_address(address);
        self.data[masked_address] = data;
    }

    pub fn chunked(&self) -> Chunks<u8> {
        self.data.chunks(16)
    }

    fn get_masked_address(&self, address: u16) -> usize {
        (address - VIDEO_RAM_LOWER) as usize
    }
}

impl Default for VideoRam {
    fn default() -> Self {
        VideoRam {
            data: [0; KILOBYTES_8 as usize],
        }
    }
}
