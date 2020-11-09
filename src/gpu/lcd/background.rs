use crate::constants::gpu::*;
use crate::constants::screen::*;
use super::Lcd;
use super::tile_data::TileData;

impl Lcd {
    pub fn render_background(&mut self) {
        let using_window = self.control.window_display() && self.window_y <= self.line_number;
        let tile_data = self.control.get_tile_data();
        let display_address = self.control.get_display_address(using_window);
        let y_position = if using_window {
            (self.line_number as u16) - (self.window_y as u16)
        } else {
            (self.scroll_y as u16) + (self.line_number as u16)
        };

        let tile_row = (y_position / PIXELS_PER_TILE) * TILE_WIDTH;
        for x in 0..SCREEN_WIDTH {
            let x_position = self.get_x_position(x, using_window);
            let tile_column = x_position / PIXELS_PER_TILE;
            let tile_address = display_address + (tile_row as u16) + (tile_column as u16);
            let tile_location = self.get_tile_location(tile_address, &tile_data);
            let color_number = self.get_color_number(tile_location, x_position, y_position);
            let color = self.bg_palette_data.get_color(color_number);

            self.screen.set_pixel(self.line_number as u16, x, color)
        }
    }

    pub fn background(&mut self) {
        let tile_map = self.control.get_tile_map();
        let tile_data = self.control.get_tile_data();
        for j in 0..256 {
            let offset_y = self.line_number + self.scroll_y;
            let offset_x = j + (self.scroll_x as u16);
            let tile_number = self.get_tile_number(tile_map, offset_y as u16, offset_x as u16);

            if tile_data.data == 0x8800 {
                let address = tile_data.data + 0x800 + ((tile_number as i8) * 0x10) + (off
                /*
colorval = 
(readFromMem(tiledata + 0x800 + ((int8_t)tilenr * 0x10) + (offY % 8 * 2)) >> (7 - (offX % 8)) & 0x1) + 
((readFromMem(tiledata + 0x800 + ((int8_t)tilenr * 0x10) + (offY % 8 * 2) + 1) >> (7 - (offX % 8)) & 0x1) * 2);
                */
            }
        }
    }

    fn get_x_position(&mut self, pixel: u16, using_window: bool) -> u16 {
        let mut x_position = pixel + (self.scroll_x as u16);
        let window_x = self.window_x as u16;
        if using_window {
            if pixel > window_x {
                x_position = pixel - window_x;
            }
        }
        
        x_position
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