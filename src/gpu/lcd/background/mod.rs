use super::Lcd;
use crate::constants::{gpu::WINDOW_X_OFFSET, screen::*};
use tile::*;

pub mod tile;

impl Lcd {
    pub fn render_background(&mut self) -> [u8; SCREEN_WIDTH as usize] {
        let window_y = self.get_window_y();
        let mut background_colors = [0; SCREEN_WIDTH as usize];

        for x in 0..SCREEN_WIDTH {
            let window_x = self.get_window_x(x);
            let tile_data = self.get_bg_tile_data(x, window_y, window_x);
            let tile_number = self.get_tile_number(&tile_data);
            let tile_address =
                tile_data.tile_base.address + tile_data.get_tile_address(tile_number);
            let color_number = self.get_bg_color_number(tile_address, &tile_data);
            let color = self.bg_palette_data.get_color(color_number);

            background_colors[x as usize] = color_number;
            self.screen.set_pixel(self.line_number as u16, x, color)
        }

        background_colors
    }

    fn get_tile_number(&self, tile_data: &TileData) -> u8 {
        let address = tile_data.get_tile_number_address();
        self.read(address)
    }

    fn get_bg_color_number(&self, tile_address: u16, tile_data: &TileData) -> u8 {
        let pixel_low = self.read(tile_address);
        let pixel_high = self.read(tile_address + 1);
        let color_bit = 7 - tile_data.pixel_x;
        let color_low = (pixel_low >> color_bit) & 0b1;
        let color_high = (pixel_high >> color_bit) & 0b1;
        (color_high << 1) | color_low
    }

    fn get_window_x(&self, x: u16) -> i32 {
        -((self.window_x as i32) - (WINDOW_X_OFFSET as i32)) + (x as i32)
    }

    fn get_window_y(&self) -> i32 {
        (self.line_number as i32) - (self.window_y as i32)
    }
}
