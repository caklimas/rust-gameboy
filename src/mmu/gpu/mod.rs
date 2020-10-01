pub mod lcd;
pub mod video_ram;

#[cfg(test)]
mod tests;

use crate::addresses::gpu::video_ram::*;
use crate::constants::gpu::*;
use lcd::lcd_mode::LcdMode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Gpu {
    horizontal_blank: bool,
    lcd_on: bool,
    line_number: u8,
    mode: LcdMode,
    mode_clock: u16,
    vertical_blank: bool,
    video_ram: video_ram::VideoRam
}

impl Gpu {
    pub fn clock(&mut self, cycles: u16) {
        if !self.lcd_on {
            return;
        }

        self.horizontal_blank = false;

        let mut ticks = cycles;
        while ticks > 0 {
            let current_ticks = if ticks >= SCAN_OAM_DOT_MAX { SCAN_OAM_DOT_MAX } else { ticks };
            self.mode_clock += current_ticks;
            ticks -= current_ticks;

            if self.mode_clock >= MODE_DOTS {
                self.mode_clock -= MODE_DOTS;
                self.line_number = (self.line_number + 1) % MAX_SCANLINE;

                if self.line_number >= VERTICAL_BLANK_SCANLINE_LOWER && self.mode != LcdMode::VerticalBlank {
                    self.set_mode(LcdMode::VerticalBlank);
                }
            }

            if self.line_number < VERTICAL_BLANK_SCANLINE_LOWER {
                if self.mode_clock <= SCAN_OAM_DOT_MAX && self.mode != LcdMode::SearchingOam {
                    self.set_mode(LcdMode::SearchingOam);
                } else if self.mode_clock <= READ_OAM_DOT_MAX && self.mode != LcdMode::TransferringDriver {
                    self.set_mode(LcdMode::TransferringDriver);
                } else if self.mode != LcdMode::HorizontalBlank {
                    self.set_mode(LcdMode::HorizontalBlank);
                }
            }
        }
    }

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

    fn set_mode(&mut self, mode: LcdMode) {
        self.mode = mode;
        match self.mode {
            LcdMode::HorizontalBlank => self.horizontal_blank = true,
            LcdMode::VerticalBlank => self.vertical_blank = true,
            _ => ()
        }
    }
}