use serde::{Serialize, Deserialize};

use crate::addresses::high_ram::*;
use crate::addresses::interrupt_enable::*;
use crate::addresses::serial_data_transfer::*;
use crate::addresses::gpu::video_ram::*;
use crate::addresses::work_ram::*;
use super::gpu;
use super::high_ram;
use super::serial_data_transfer::SerialDataTransfer;
use super::work_ram;

#[derive(Serialize, Deserialize, Default)]
pub struct Ram {
    gpu: gpu::Gpu, 
    high_ram: high_ram::HighRam,
    interrupt_enable: u8,
    interrupt_flag: u8,
    serial_data_transfer: SerialDataTransfer,
    work_ram: work_ram::WorkRam
}

impl Ram {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.gpu.read(address),
            WORK_RAM_LOWER..=WORK_RAM_UPPER => self.work_ram.read(address),
            SERIAL_TRANSFER_DATA..=SERIAL_TRANSFER_CONTROL => self.serial_data_transfer.read(address),
            INTERRUPT_FLAG => self.interrupt_flag,
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.read(address),
            INTERRUPT_ENABLE => self.interrupt_enable,
            _ => {
                // println!("Invalid address 0x{:4X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.gpu.write(address, data),
            WORK_RAM_LOWER..=WORK_RAM_UPPER => self.work_ram.write(address, data),
            SERIAL_TRANSFER_DATA..=SERIAL_TRANSFER_CONTROL => self.serial_data_transfer.write(address, data),
            INTERRUPT_FLAG => self.interrupt_flag = data,
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.write(address, data),
            INTERRUPT_ENABLE => self.interrupt_enable = data,
            _ => () // println!("Invalid address 0x{:4X}", address)
        }
    }
}