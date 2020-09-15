use serde::{Serialize, Deserialize};

use super::super::addresses::high_ram::*;
use super::super::addresses::interrupt_enable::INTERRUPT_ENABLE_REGISTER;
use super::super::addresses::video_ram::*;
use super::super::addresses::work_ram::*;
use super::high_ram;
use super::video_ram;
use super::work_ram;

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
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.read(address),
            WORK_RAM_LOWER..=WORK_RAM_UPPER => self.work_ram.read(address),
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.read(address),
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable,
            _ => {
                println!("Invalid address 0x{:4X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.write(address, data),
            WORK_RAM_LOWER..=WORK_RAM_UPPER => self.work_ram.write(address, data),
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.write(address, data),
            INTERRUPT_ENABLE_REGISTER => self.interrupt_enable = data,
            _ => println!("Invalid address 0x{:4X}", address)
        }
    }
}