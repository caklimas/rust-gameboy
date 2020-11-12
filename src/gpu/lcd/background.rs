use crate::constants::screen::*;
use super::Lcd;
use super::tile::*;

impl Lcd {
    pub fn render_background(&mut self) {
        let using_window = self.control.window_display() && self.window_y <= self.line_number;
        for x in 0..SCREEN_WIDTH {
            let tile_data = self.get_tile_data(x, using_window);
            let tile_number = self.get_tile_number(&tile_data);
            let tile_address = tile_data.tile_base.address + tile_data.get_tile_address(tile_number);
            let color_number = self.get_color_number(tile_address, &tile_data);
            let color = self.bg_palette_data.get_color(color_number);

            self.screen.set_pixel(self.line_number as u16, x, color)
        }
    }

    fn get_tile_number(&self, tile_data: &TileData) -> u8 {
        let address = tile_data.get_tile_number_address();
        self.read(address)
    }

    fn get_color_number(&self, tile_address: u16, tile_data: &TileData) -> u8 {
        let pixel_low = self.read(tile_address);
        let pixel_high = self.read(tile_address + 1);
        let color_bit = 7 - tile_data.pixel_x;
        let color_low = (pixel_low >> color_bit) & 0b1;
        let color_high = (pixel_high >> color_bit) & 0b1;
        (color_high << 1) | color_low
    }
}