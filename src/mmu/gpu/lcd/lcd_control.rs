use serde::{Serialize, Deserialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct LcdControl(u8);
    impl Debug;

    pub bg_window_display_priority, set_bg_window_display_priority: 0;
    pub sprite_display, set_sprite_display: 1;
    pub sprite_size, set_sprite_size: 2;
    pub bg_tile_map_display_select, set_bg_tile_map_display_select: 3;
    pub bg_window_tile_data_select, set_bg_window_tile_data_select: 4;
    pub window_display, set_window_display: 5;
    pub window_tile_map_display_select, set_window_tile_map_display_select: 6;
    pub lcd_display_enable, set_lcd_display_enable: 7;
    pub get, set: 7, 0;
}