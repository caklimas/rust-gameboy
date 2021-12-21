use crate::addresses::serial_data_transfer::*;
use serde::{Deserialize, Serialize};

pub mod serial_transfer_control;

#[derive(Serialize, Deserialize, Default)]
pub struct SerialDataTransfer {
    control: serial_transfer_control::SerialTransferControl,
    data: u8,
}

impl SerialDataTransfer {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            SERIAL_TRANSFER_DATA => self.data,
            SERIAL_TRANSFER_CONTROL => self.control.get(),
            _ => panic!("Invalid serial data address: 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            SERIAL_TRANSFER_DATA => {
                print!("{}", data as char);
                self.data = data;
            }
            SERIAL_TRANSFER_CONTROL => self.control.set(data),
            _ => panic!("Invalid serial data address: 0x{:4X}", address),
        }
    }
}
