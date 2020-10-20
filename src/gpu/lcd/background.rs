use crate::constants::screen::*;
use super::Lcd;
use super::tile_data::TileData;

impl Lcd {
    pub fn render_background(&mut self) {
        let using_window = self.control.window_display() && self.window_y <= self.line_number;
        let tile_data = self.control.get_tile_data();
        let display_address = self.control.get_display_address(using_window);
        let y_position = if using_window {
            self.line_number - self.window_y
        } else {
            self.scroll_y + self.line_number
        };

        let tile_row = (y_position / PIXELS_PER_TILE) * TILE_WIDTH;
        for x in 0..SCREEN_WIDTH {
            let x_position = if using_window {
                x - self.window_x
            } else {
                self.scroll_x + x
            };

            let tile_column = x_position / PIXELS_PER_TILE;
            let tile_address = display_address + (tile_row as u16) + (tile_column as u16);
            let tile_location = self.get_tile_location(tile_address, &tile_data);
            let color_number = self.get_color_number(tile_location, x_position, y_position);
            let color = self.bg_palette_data.get_color(color_number);
            self.screen.set_pixel(self.line_number, x, color)
        }
    }

    fn get_color_number(&self, tile_location: u16, x_position: u8, y_position: u8) -> u8 {
        let line = ((y_position % 8) * 2) as u16; // vertical lines take up 2 bytes of memory
        let pixel_low = self.read(tile_location + line);
        let pixel_high = self.read(tile_location + line + 1);
        let color_bit = (((x_position as i8) % 8) - 7) * -1;
        let color_low = (pixel_low >> color_bit) & 0b1;
        let color_high = (pixel_high >> color_bit) & 0b1;
        (color_high << 1) | color_low
    }

    fn get_tile_location(&self, address: u16, tile_data: &TileData) -> u16 {
        if tile_data.use_unsigned {
            self.get_tile_location_u8(address, tile_data.data)
        } else {
            self.get_tile_location_i8(address, tile_data.data)
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