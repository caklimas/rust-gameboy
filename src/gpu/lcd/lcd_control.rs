use serde::{Serialize, Deserialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct LcdControl(u8);
    impl Debug;

    pub background_enabled, set_background_enabled: 0;
    pub sprite_enabled, set_sprite_enabled: 1;
    pub sprite_size, set_sprite_size: 2;
    pub bg_tile_map_display_select, set_bg_tile_map_display_select: 3; // 0=9800-9BFF, 1=9C00-9FFF
    pub bg_window_tile_data_select, set_bg_window_tile_data_select: 4; // 0=8800-97FF, 1=8000-8FFF
    pub window_display, set_window_display: 5;
    pub window_tile_map_display_select, set_window_tile_map_display_select: 6; // 0=9800-9BFF, 1=9C00-9FFF
    pub lcd_display_enable, set_lcd_display_enable: 7;
    pub get, set: 7, 0;
}