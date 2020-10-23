use crate::gpu::lcd::palette::Rgb;

pub const MAX_SCANLINE: u8 = 154;
pub const VERTICAL_BLANK_SCANLINE_LOWER: u8 = 144;

// Mode cycles
pub const MODE_CYCLES: u16 = 456;
pub const SEARCH_OAM_CYCLES: u16 = 80;
pub const DRAWING_CYCLES: u16 = 172;
pub const HORIZONTAL_BLANK_CYCLES: u16 = 204;

// Window
pub const WINDOW_X_OFFSET: u8 = 7;

// Colors
pub const COLOR_PER_PIXEL: usize = 3;
pub const RGB_BLACK: Rgb = (0, 0, 0);
pub const RGB_DARK_GRAY: Rgb = (96, 96, 96);
pub const RGB_LIGHT_GRAY: Rgb = (192, 192, 192);
pub const RGB_WHITE: Rgb = (255, 255, 255);