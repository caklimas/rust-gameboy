use super::Lcd;
use crate::constants::lcd::*;
use crate::constants::screen::*;

impl Lcd {
    pub fn get_bg_tile_data(&self, x: u16, window_y: i32, window_x: i32) -> TileData {
        let using_window = self.control.window_display() && window_y >= 0 && window_x >= 0;
        let tile_base = TileBase::new(self.control.bg_window_tile_data_select());

        TileData::new(
            self.get_pixel_x(x, using_window, window_x),
            self.get_pixel_y(using_window, window_y),
            tile_base,
            self.get_display_address(using_window),
            self.get_tile_x(x, using_window, window_x),
            self.get_tile_y(using_window, window_y),
        )
    }

    fn get_pixel_x(&self, x: u16, using_window: bool, window_x: i32) -> u8 {
        let pixel_x = if using_window {
            window_x as u8
        } else {
            self.get_background_x(x) as u8
        };

        pixel_x & 0x07
    }

    fn get_pixel_y(&self, using_window: bool, window_y: i32) -> u16 {
        let pixel_y = if using_window {
            window_y as u16
        } else {
            self.get_background_y() as u16
        };

        pixel_y & 0x07
    }

    fn get_tile_x(&self, x: u16, using_window: bool, window_x: i32) -> u16 {
        let tile = if using_window {
            window_x as u16
        } else {
            self.get_background_x(x) as u16
        };

        let mut tile_x = tile / PIXELS_PER_TILE;
        if !using_window {
            tile_x = tile_x & 31;
        }

        tile_x
    }

    fn get_tile_y(&self, using_window: bool, window_y: i32) -> u16 {
        let tile_y = if using_window {
            window_y as u16
        } else {
            self.get_background_y() as u16
        };

        (tile_y / PIXELS_PER_TILE) % TILE_WIDTH
    }

    fn get_background_x(&self, x: u16) -> u32 {
        (x as u32) + (self.scroll_x as u32)
    }

    fn get_background_y(&self) -> u8 {
        self.scroll_y.wrapping_add(self.line_number)
    }

    fn get_display_address(&self, using_window: bool) -> u16 {
        let display_select = if using_window {
            self.control.window_tile_map_display_select()
        } else {
            self.control.bg_tile_map_display_select()
        };

        if display_select {
            0x9C00
        } else {
            0x9800
        }
    }
}

pub struct TileData {
    pub pixel_x: u8,
    pub pixel_y: u16,
    pub tile_base: TileBase,
    pub tile_map_base: u16,
    pub tile_x: u16,
    pub tile_y: u16,
}

impl TileData {
    pub fn new(
        pixel_x: u8,
        pixel_y: u16,
        tile_base: TileBase,
        tile_map_base: u16,
        tile_x: u16,
        tile_y: u16,
    ) -> Self {
        TileData {
            pixel_x,
            pixel_y,
            tile_base,
            tile_map_base,
            tile_x,
            tile_y,
        }
    }

    pub fn get_tile_number_address(&self) -> u16 {
        self.tile_map_base + (self.tile_y * TILE_WIDTH) + self.tile_x
    }

    pub fn get_tile_address(&self, tile_number: u8) -> u16 {
        let tile_address = if self.tile_base.use_unsigned {
            tile_number as u16
        } else {
            (tile_number as i8 as i16 + (TILE_MEMORY_OFFSET as i16)) as u16
        };

        (tile_address * TILE_SIZE) + (self.pixel_y * 2)
    }
}

pub struct TileBase {
    pub address: u16,
    pub use_unsigned: bool,
}

impl TileBase {
    pub fn new(bg_window_tile_data_select: bool) -> Self {
        if bg_window_tile_data_select {
            TileBase {
                address: 0x8000,
                use_unsigned: true,
            }
        } else {
            TileBase {
                address: 0x8800,
                use_unsigned: false,
            }
        }
    }
}
