use serde::{Serialize, Deserialize};
use crate::addresses::gpu::lcd::*;
use crate::addresses::gpu::video_ram::*;
use crate::constants::gpu::*;

pub mod background;
pub mod bg_palette_data;
pub mod lcd_control;
pub mod lcd_mode;
pub mod lcd_status;
pub mod obj_palette_data;
pub mod palette;
pub mod screen;
pub mod tile_data;

use lcd_mode::LcdMode;
use crate::mmu::interrupts::lcd_interrupt::LcdInterruptResult;
use super::video_ram::VideoRam;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Lcd {
    pub frame_complete: bool,
    pub screen: screen::Screen,
    bg_palette_data: bg_palette_data::BgPaletteData,
    control: lcd_control::LcdControl,
    line_number: u8,
    lyc: u8,
    mode: LcdMode,
    mode_clock: u16,
    obj_palette_0_data: obj_palette_data::ObjPaletteData,
    obj_palette_1_data: obj_palette_data::ObjPaletteData,
    scroll_x: u8,
    scroll_y: u8,
    status: lcd_status::LcdStatus,
    window_x: u8,
    window_y: u8,
    video_ram: VideoRam
}

impl Lcd {
    pub fn clock(&mut self, cycles: u16) -> LcdInterruptResult {
        let result = LcdInterruptResult::new();
        if !self.control.lcd_display_enable() {
            return result;
        }

        self.mode_clock += cycles;
        match self.mode {
            LcdMode::SearchingOam => {
                if self.mode_clock >= SEARCH_OAM_CYCLES {
                    self.set_mode(LcdMode::Drawing);
                } 
            },
            LcdMode::Drawing => {
                if self.mode_clock >= DRAWING_CYCLES {
                    self.set_mode(LcdMode::HorizontalBlank);
                    self.render_scanline();
                }
            },
            LcdMode::HorizontalBlank => {
                if self.mode_clock >= HORIZONTAL_BLANK_CYCLES {
                    self.line_number += 1;

                    if self.line_number == VERTICAL_BLANK_SCANLINE_LOWER - 1 {
                        self.set_mode(LcdMode::VerticalBlank);
                        self.frame_complete = true;
                    } else {
                        self.set_mode(LcdMode::SearchingOam);
                        self.frame_complete = false;
                    }
                }
            },
            LcdMode::VerticalBlank => {
                if self.mode_clock >= MODE_CYCLES {
                    self.mode_clock = 0;
                    self.line_number += 1;

                    if self.line_number >= MAX_SCANLINE {
                        self.set_mode(LcdMode::SearchingOam);
                        self.line_number = 0;
                    }
                }
            }
        }

        result
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            LCD_CONTROL => self.control.get(),
            LCD_STATUS => self.status.0,
            LCD_SCROLL_Y => self.scroll_y,
            LCD_SCROLL_X => self.scroll_x,
            LCD_LY => self.line_number,
            LCD_LYC => self.lyc,
            LCD_BG_PALETTE_DATA => self.bg_palette_data.into_u8(),
            LCD_OBJ_0_PALETTE_DATA => self.obj_palette_0_data.into_u8(),
            LCD_OBJ_1_PALETTE_DATA => self.obj_palette_1_data.into_u8(),
            LCD_WINDOW_Y => self.window_y,
            LCD_WINDOW_X => self.window_x,
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.read(address),
            _ => panic!("Invalid lcd address: 0x{:4X}", address)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            LCD_CONTROL => self.control.set(data),
            LCD_STATUS => self.set_status(data),
            LCD_SCROLL_Y => self.scroll_y = data,
            LCD_SCROLL_X => self.scroll_x = data,
            LCD_LY => (), // readonly
            LCD_LYC => self.lyc = data,
            LCD_BG_PALETTE_DATA => self.bg_palette_data = bg_palette_data::BgPaletteData::from_u8(data),
            LCD_OBJ_0_PALETTE_DATA => self.obj_palette_0_data = obj_palette_data::ObjPaletteData::from_u8(data),
            LCD_OBJ_1_PALETTE_DATA => self.obj_palette_1_data = obj_palette_data::ObjPaletteData::from_u8(data),
            LCD_WINDOW_Y => self.window_y = data,
            LCD_WINDOW_X => self.window_x = data - WINDOW_X_OFFSET,
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.write(address, data),
            _ => panic!("Invalid lcd address: 0x{:4X}", address)
        }
    }

    fn render_scanline(&mut self) {
        if self.control.background_enabled() {
            self.render_background();
        }

        if self.control.sprite_enabled() {
            self.render_sprites();
        }
    }

    fn render_sprites(&mut self) {

    }

    fn set_mode(&mut self, mode: LcdMode) {
        self.mode_clock = 0;
        self.mode = mode;
    }

    fn set_status(&mut self, data: u8) {
        self.status.set(data);        
        self.status.set_line_coincidence(self.line_number == self.lyc);
    }
}