use crate::constants::gpu::*;
use crate::constants::screen::*;
use super::Lcd;

impl Lcd {
    pub fn get_tile_data(&self, y: u16, x: u16, using_window: bool) {
        let tile_base = TileBase::new(self.control.bg_window_tile_data_select());        
        let tile_x = self.get_tile_x(x, using_window);
    }

    fn get_tile_x(&self, x: u16, using_window: bool) -> u16 {
        let tile_x = if using_window {
            self.get_window_x(x) as u16
        } else {
            x + (self.scroll_x as u16)
        };

        tile_x / PIXELS_PER_TILE
    }

    fn get_tile_y(&self, using_window: bool) -> u16 {
        let tile_y = if using_window {
            ((self.line_number as i32) - (self.window_y as i32)) as u16
        } else {
            self.scroll_y.wrapping_add(self.line_number) as u16
        };

        (tile_y / PIXELS_PER_TILE) % TILE_WIDTH
    }

    fn get_window_x(&self, x: u16) -> i32 {
        - ((self.window_x as i32) - 7) + (x as i32)
    }
}

pub struct TileData {
    pub pixel_x: u16,
    pub pixel_y: u16,
    pub tile_base: TileBase,
    pub tile_map_base: u16,
    pub tile_x: u16,
    pub tile_y: u16
}

impl TileData {
    pub fn new(
        pixel_x: u16,
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
        if self.bg_window_tile_data_select() {
            TileBase { address: 0x8000, use_unsigned: true }
        } else {
            TileBase { address: 0x8800, use_unsigned: false }
        }
    }
}