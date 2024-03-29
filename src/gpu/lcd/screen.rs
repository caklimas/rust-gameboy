use crate::constants::gpu::*;
use crate::constants::screen::*;
use crate::gpu::lcd::palette::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Screen {
    pixels: Vec<u8>,
}

impl Screen {
    pub fn new() -> Self {
        let size = COLOR_PER_PIXEL * (SCREEN_HEIGHT * SCREEN_WIDTH) as usize;
        Screen {
            pixels: vec![255; size],
        }
    }

    pub fn get_pixels(&self) -> &[u8] {
        &self.pixels
    }

    pub fn set_pixel(&mut self, y: u16, x: u16, color: Rgb) {
        if y >= SCREEN_HEIGHT || x >= SCREEN_WIDTH {
            return;
        }

        let y_index = ((SCREEN_WIDTH * y) as usize) * COLOR_PER_PIXEL;
        let x_index = y_index + ((x as usize) * COLOR_PER_PIXEL); // For RGB
        self.pixels[x_index] = color.0;
        self.pixels[x_index + 1_usize] = color.1;
        self.pixels[x_index + 2_usize] = color.2;
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}
