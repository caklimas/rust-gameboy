pub mod lcd_interrupt;

use serde::{Serialize, Deserialize};

bitfield!{
    #[derive(Serialize, Deserialize, Default)]
    pub struct Interrupt(u8);
    impl Debug;

    pub v_blank, set_v_blank: 0;
    pub lcd_stat, set_lcd_stat: 1;
    pub timer, set_timer: 2;
    pub serial, set_serial: 3;
    pub joypad, set_joypad: 4;
    pub get, set: 4, 0;
}