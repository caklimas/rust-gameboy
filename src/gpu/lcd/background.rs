use crate::constants::gpu::*;
use crate::constants::screen::*;
use super::Lcd;
use super::tile::*;

impl Lcd {
    pub fn render_background(&mut self) {
        let using_window = self.control.window_display() && self.window_y <= self.line_number;
        for x in 0..SCREEN_WIDTH {
            let tile_data = self.get_tile_data(x, using_window);
            // let color_number = self.get_color_number(tile_location, x_position, y_position);
            // let color = self.bg_palette_data.get_color(color_number);

            // self.screen.set_pixel(self.line_number as u16, x, color)
        }
    }

    fn get_y_position(&self, using_window: bool) -> u16 {
        if using_window {
            (self.line_number as u16) - (self.window_y as u16)
        } else {
            (self.scroll_y as u16) + (self.line_number as u16)
        }
    }

    fn get_color_number(&self, tile_location: u16, x_position: u16, y_position: u16) -> u8 {
        let line = ((y_position % 8) * 2) as u16; // vertical lines take up 2 bytes of memory
        let pixel_address = tile_location + line;
        let pixel_low = self.read(pixel_address);
        let pixel_high = self.read(pixel_address + 1);
        let color_bit = (((x_position as i16) % 8) - (WINDOW_X_OFFSET as i16)) * -1;
        let color_low = (pixel_low >> color_bit) & 0b1;
        let color_high = (pixel_high >> color_bit) & 0b1;
        (color_high << 1) | color_low
    }

    fn get_tile_number(&self, tile_map: u16, offset_y: u16, offset_x: u16) -> u8 {
        let address = tile_map + ((offset_y / PIXELS_PER_TILE * TILE_WIDTH) + (offset_x / PIXELS_PER_TILE));
        self.read(address)
    }

    fn get_tile_location(&self, address: u16, tile_data: &TileData) -> u16 {
        if tile_data.tile_base.use_unsigned {
            self.get_tile_location_u8(address, tile_data.tile_base.address)
        } else {
            self.get_tile_location_i8(address, tile_data.tile_base.address)
        }
    }

    fn get_tile_location_i8(&self, address: u16, tile_data: u16) -> u16 {
        let tile_number = self.read(address) as i8;
        tile_data + ((tile_number as i8 as i16 + 128) as u16 * 16)
    }

    fn get_tile_location_u8(&self, address: u16, tile_data: u16) -> u16 {
        let tile_number = self.read(address) as u16;
        tile_data + (tile_number * 16)
    }
}