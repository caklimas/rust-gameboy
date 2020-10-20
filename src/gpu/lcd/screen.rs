use serde::{Serialize, Deserialize};
use crate::constants::gpu::*;
use crate::constants::screen::*;
use crate::gpu::lcd::palette::Rgb;

#[derive(Serialize, Deserialize)]
pub struct Screen {
    pixels: Vec<Vec<Rgb>>
}

impl Screen {
    pub fn new() -> Self {
        let mut pixels: Vec<Vec<Rgb>> = Vec::new();
        for _ in 0..SCREEN_HEIGHT {
            pixels.push(vec![RGB_WHITE; SCREEN_WIDTH as usize]);
        }

        Screen {
            pixels
        }
    }

    pub fn set_pixel(&mut self, y: u16, x: u16, color: Rgb) {
        if y >= SCREEN_WIDTH || x >= SCREEN_HEIGHT {
            return;
        }

        self.pixels[y as usize][x as usize] = color;
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}