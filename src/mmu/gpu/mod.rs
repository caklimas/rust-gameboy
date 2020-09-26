use serde::{Serialize, Deserialize};
use crate::addresses::gpu::video_ram::*;

pub mod lcd;
pub mod video_ram;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Gpu {
    video_ram: video_ram::VideoRam
}

impl Gpu {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.read(address),
            _ => panic!("Invalid GPU address 0x{:4X}", address)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.write(address, data),
            _ => panic!("Invalid GPU address 0x{:4X}", address)
        }
    }
}