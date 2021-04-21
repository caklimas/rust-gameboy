use serde::{Serialize, Deserialize};

use crate::addresses::{high_ram::*};
use crate::addresses::interrupt_enable::*;
use crate::addresses::serial_data_transfer::*;
use crate::addresses::gpu::lcd::*;
use crate::addresses::gpu::video_ram::*;
use crate::addresses::gpu::sprite::*;
use crate::addresses::timer::*;
use crate::addresses::work_ram::*;
use crate::gpu;
use super::high_ram;
use super::interrupts::Interrupt;
use super::serial_data_transfer::SerialDataTransfer;
use super::timer::Timer;
use super::work_ram;

#[derive(Serialize, Deserialize, Default)]
pub struct Ram {
    pub gpu: gpu::Gpu, 
    pub interrupt_enable: Interrupt,
    pub interrupt_flag: Interrupt,
    high_ram: high_ram::HighRam,
    serial_data_transfer: SerialDataTransfer,
    timer: Timer,
    work_ram: work_ram::WorkRam
}

impl Ram {
    pub fn clock(&mut self, cycles: u16) {
        let gpu_cycles = cycles; 
        self.clock_timer(gpu_cycles);
        self.clock_gpu(gpu_cycles);
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.gpu.read(address),
            WORK_RAM_LOWER..=WORK_RAM_UPPER => self.work_ram.read(address),
            SERIAL_TRANSFER_DATA..=SERIAL_TRANSFER_CONTROL => self.serial_data_transfer.read(address),
            DIVIDER_REGISTER..=TIMER_CONTROL => self.timer.read(address),
            INTERRUPT_FLAG => self.interrupt_flag.get(),
            LCD_DMA_START => 0, // write only
            LCD_CONTROL..=LCD_LYC | LCD_BG_PALETTE_DATA..=LCD_WINDOW_X => self.gpu.read(address),
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.read(address),
            SPRITE_ATTRIBUTE_TABLE_LOWER..=SPRITE_ATTRIBUTE_TABLE_UPPER => self.gpu.read(address),
            INTERRUPT_ENABLE => self.interrupt_enable.get(),
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
            DIVIDER_REGISTER..=TIMER_CONTROL => self.timer.write(address, data),
            INTERRUPT_FLAG => self.interrupt_flag.set(data),
            LCD_DMA_START => self.run_dma(data),
            LCD_CONTROL..=LCD_LYC | LCD_BG_PALETTE_DATA..=LCD_WINDOW_X => self.gpu.write(address, data),
            HIGH_RAM_LOWER..=HIGH_RAM_UPPER => self.high_ram.write(address, data),
            SPRITE_ATTRIBUTE_TABLE_LOWER..=SPRITE_ATTRIBUTE_TABLE_UPPER => self.gpu.write(address, data),
            INTERRUPT_ENABLE => self.interrupt_enable.set(data),
            _ => () // println!("Invalid address 0x{:4X}", address)
        }
    }

    fn clock_gpu(&mut self, cycles: u16) {
        let result = self.gpu.clock(cycles);
        self.interrupt_flag.set_lcd_stat(result.lcd_stat);
        self.interrupt_flag.set_v_blank(result.vertical_blank);
    }

    fn clock_timer(&mut self, cycles: u16) {
        self.timer.clock(cycles);
        if self.timer.interrupt_requested {
            self.interrupt_flag.set_timer(true);
            self.timer.interrupt_requested = false;
        }
    }

    fn run_dma(&mut self, data: u8) {
        let base_address = (data as u16) << 8;
        for i in 0..=0x9F {
            let sprite_data = self.read(base_address + i);
            let sprite_address = SPRITE_ATTRIBUTE_TABLE_LOWER + i;
            self.write(sprite_address, sprite_data);
        }
    }
}