use crate::addresses::gpu::lcd::*;
use crate::addresses::gpu::sprite::*;
use crate::addresses::gpu::video_ram::*;
use crate::constants::gpu::*;
use serde::{Deserialize, Serialize};

pub mod background;
pub mod bg_palette_data;
pub mod lcd_control;
pub mod lcd_mode;
pub mod lcd_status;
pub mod obj_palette_data;
pub mod palette;
pub mod screen;
pub mod sprites;

use super::{video_oam::VideoOam, video_ram::VideoRam};
use crate::mmu::interrupts::lcd_interrupt::LcdInterruptResult;
use lcd_mode::LcdMode;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Lcd {
    pub frame_complete: bool,
    pub screen: screen::Screen,
    pub video_ram: VideoRam,
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
    window_line_counter: u8,
    window_x: u8,
    window_y: u8,
    video_oam: VideoOam,
}

impl Lcd {
    pub fn clock(&mut self, cycles: u16) -> LcdInterruptResult {
        let mut result = LcdInterruptResult::default();
        if !self.control.lcd_display_enable() {
            return result;
        }

        self.mode_clock += cycles;
        match self.mode {
            LcdMode::SearchingOam => {
                if self.mode_clock >= SEARCH_OAM_CYCLES {
                    self.set_mode(LcdMode::Drawing);
                }
            }
            LcdMode::Drawing => {
                if self.mode_clock >= DRAWING_CYCLES {
                    self.render_scanline();
                    self.set_mode(LcdMode::HorizontalBlank);

                    if self.window_visible() {
                        self.window_line_counter += 1;
                    }

                    self.check_lyc_interrupt(&mut result);
                    self.line_number = (self.line_number + 1) % 154;

                    if self.status.horizontal_blank_interrupt() {
                        result.lcd_stat = true;
                    }
                }
            }
            LcdMode::HorizontalBlank => {
                if self.mode_clock >= HORIZONTAL_BLANK_CYCLES {
                    if self.line_number >= VERTICAL_BLANK_SCANLINE_LOWER {
                        self.set_mode(LcdMode::VerticalBlank);
                        result.vertical_blank = true;
                        self.frame_complete = true;

                        if self.status.vertical_blank_interrupt() {
                            result.lcd_stat = true;
                        }
                    } else {
                        self.set_mode(LcdMode::SearchingOam);
                        self.frame_complete = false;

                        if self.status.oam_interrupt() {
                            result.lcd_stat = true;
                        }
                    }
                    self.check_lyc_interrupt(&mut result);
                }
            }
            LcdMode::VerticalBlank => {
                self.frame_complete = false;
                if self.mode_clock >= MODE_CYCLES {
                    self.mode_clock = 0;
                    self.line_number += 1;

                    if self.line_number >= MAX_SCANLINE {
                        self.set_mode(LcdMode::SearchingOam);
                        self.line_number = 0;
                        self.window_line_counter = 0;
                        self.check_lyc_interrupt(&mut result);
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
            SPRITE_ATTRIBUTE_TABLE_LOWER..=SPRITE_ATTRIBUTE_TABLE_UPPER => {
                self.video_oam.read(address)
            }
            _ => panic!("Invalid lcd address: 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            LCD_CONTROL => {
                let previous_lcd_on = self.control.lcd_display_enable();
                self.control.set(data);
                if previous_lcd_on && !self.control.lcd_display_enable() {
                    info!("Turned off");
                    self.mode_clock = 0;
                    self.mode = LcdMode::HorizontalBlank;
                    self.line_number = 0;
                } else if !previous_lcd_on && self.control.lcd_display_enable() {
                    info!("Turned on");
                }
            }
            LCD_STATUS => self.set_status(data),
            LCD_SCROLL_Y => self.scroll_y = data,
            LCD_SCROLL_X => self.scroll_x = data,
            LCD_LY => (), // readonly
            LCD_LYC => self.lyc = data,
            LCD_BG_PALETTE_DATA => {
                self.bg_palette_data = bg_palette_data::BgPaletteData::from_u8(data)
            }
            LCD_OBJ_0_PALETTE_DATA => {
                self.obj_palette_0_data = obj_palette_data::ObjPaletteData::from_u8(data)
            }
            LCD_OBJ_1_PALETTE_DATA => {
                self.obj_palette_1_data = obj_palette_data::ObjPaletteData::from_u8(data)
            }
            LCD_WINDOW_Y => self.window_y = data,
            LCD_WINDOW_X => self.window_x = data,
            VIDEO_RAM_LOWER..=VIDEO_RAM_UPPER => self.video_ram.write(address, data),
            SPRITE_ATTRIBUTE_TABLE_LOWER..=SPRITE_ATTRIBUTE_TABLE_UPPER => {
                self.video_oam.write(address, data)
            }
            _ => panic!("Invalid lcd address: 0x{:4X}", address),
        }
    }

    /**
     * https://www.huderlem.com/demos/gameboy2bpp.html
     */
    pub fn render_vram(&self) -> Vec<u8> {
        // Every 16 bytes fully represent a tile
        let chunks = self.video_ram.chunked();
        let mut colors = Vec::new();

        for chunk in chunks {
            // Each pair of bytes represent a row
            let rows = chunk.chunks(2);
            for row in rows {
                // Combine each bit of high and each bit of low to determine the color number
                let low_byte = row[0];
                let high_byte = row[1];

                // Start from left most bit
                for i in 0..8 {
                    let high_bit = (high_byte >> (7 - i)) & 0b1;
                    let low_bit = (low_byte >> (7 - i)) & 0b1;

                    // The color is {high_bit}{low_bit} ex. if high is 0 and low is 1 then color is 01
                    let color_number = (high_bit << 1) | low_bit;
                    let color = self.bg_palette_data.get_color(color_number);
                    colors.push(color.0);
                    colors.push(color.1);
                    colors.push(color.2);
                }
            }
        }

        colors
    }

    fn render_scanline(&mut self) {
        let mut background_colors = Option::None;
        if self.control.background_enabled() {
            background_colors = Option::Some(self.render_background());
        }

        if self.control.sprite_enabled() {
            self.render_sprites(background_colors);
        }
    }

    fn set_mode(&mut self, mode: LcdMode) {
        if self.mode == mode {
            return;
        }

        self.mode_clock = 0;
        self.mode = mode;
    }

    fn set_status(&mut self, data: u8) {
        self.status.set(data);
        self.status
            .set_line_coincidence(self.line_number == self.lyc);
    }

    fn check_lyc_interrupt(&mut self, result: &mut LcdInterruptResult) {
        if self.line_number == self.lyc {
            result.lcd_stat = true;
        }
    }

    fn window_visible(&self) -> bool {
        self.control.window_display()
            && self.window_x < 166
            && self.window_y < 143
            && self.line_number >= self.window_y
    }
}
