use crate::constants::screen::*;
use super::Lcd;

impl Lcd {
    pub fn get_tile_data(&self, x: u16, using_window: bool) -> TileData {
        let tile_base = TileBase::new(self.control.bg_window_tile_data_select());        

        TileData::new(
            self.get_pixel_x(x, using_window),
            self.get_pixel_y(using_window),
            tile_base,
            self.get_display_address(using_window),
            self.get_tile_x(x, using_window),
            self.get_tile_y(using_window)
        )
    }

    fn get_pixel_x(&self, x: u16, using_window: bool) -> u8 {
        let pixel_x = if using_window {
            self.get_window_x(x) as u8
        } else {
            self.get_background_x(x) as u8
        };

        pixel_x & 0x07
    }

    fn get_pixel_y(&self, using_window: bool) -> u16 {
        let pixel_y = if using_window {
            self.get_window_y() as u16
        } else {
            self.get_background_y() as u16
        };

        pixel_y & 0x07
    }

    fn get_tile_x(&self, x: u16, using_window: bool) -> u16 {
        let tile_x = if using_window {
            self.get_window_x(x) as u16
        } else {
            self.get_background_x(x) as u16
        };

        tile_x / PIXELS_PER_TILE
    }

    fn get_tile_y(&self, using_window: bool) -> u16 {
        let tile_y = if using_window {
            self.get_window_y() as u16
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

    fn get_window_x(&self, x: u16) -> i32 {
        - ((self.window_x as i32) - 7) + (x as i32)
    }

    fn get_window_y(&self) -> i32 {
        (self.line_number as i32) - (self.window_y as i32)
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
    pub tile_y: u16
}

impl TileData {
    pub fn new(
        pixel_x: u8,
        pixel_y: u16,
        tile_base: TileBase,
        tile_map_base: u16,
        tile_x: u16,
        tile_y: u16
    ) -> Self {
        TileData {
            pixel_x,
            pixel_y,
            tile_base,
            tile_map_base,
            tile_x,
            tile_y
        }
    }
}

pub struct TileBase {
    pub address: u16,
    pub use_unsigned: bool
}

impl TileBase {
    pub fn new(bg_window_tile_data_select: bool) -> Self {
        if bg_window_tile_data_select {
            TileBase { address: 0x8000, use_unsigned: true }
        } else {
            TileBase { address: 0x8800, use_unsigned: false }
        }
    }
}