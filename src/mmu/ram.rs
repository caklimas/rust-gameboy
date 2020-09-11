use serde::{Serialize, Deserialize};

use super::super::addresses::high_ram::*;
use super::super::addresses::interrupt_enable::INTERRUPT_ENABLE_REGISTER;
use super::super::addresses::video_ram::*;
use super::high_ram;
use super::video_ram;
use super::work_ram;

const logo: [u8; 48] = [0xce, 0xed, 0x66, 0x66, 0xcc, 0x0d, 0x00, 0x0b, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0c, 0x00, 0x0d, 0x00, 0x08, 0x11, 0x1f, 0x88, 0x89, 0x00, 0x0e, 0xdc, 0xcc, 0x6e, 0xe6, 0xdd, 0xdd, 0xd9, 0x99, 0xbb, 0xbb, 0x67, 0x63, 0x6e, 0x0e, 0xec, 0xcc, 0xdd, 0xdc, 0x99, 0x9f, 0xbb, 0xb9, 0x33, 0x3e];

#[derive(Serialize, Deserialize, Default)]
pub struct Ram {
    high_ram: high_ram::HighRam,
    interrupt_enable: u8,
    video_ram: video_ram::VideoRam,
    work_ram: work_ram::WorkRam
}

impl Ram {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            0x104..=0x133 => logo[(address % 0x104) as usize],
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.read(address),
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.read(address),
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable,
            _ => {
                // println!("Invalid address 0x{:4X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        if address == 0x0134 {
            let x = 5;
        }

        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.write(address, data),
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.write(address, data),
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable = data,
            _ => () // println!("Invalid address 0x{:4X}", address)
        }
    }
}