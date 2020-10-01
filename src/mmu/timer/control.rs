use serde::{Serialize, Deserialize};
use crate::constants::cpu::CPU_REFRESH_RATE;

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct TimerControl(u8);
    impl Debug;

    pub input_clock_select, _: 1, 0;
    pub timer_enable, _: 2;
    pub get, set: 2, 0;
}

impl TimerControl {
    pub fn get_input_clock_select(&self) -> u32 {
        match self.input_clock_select() {
            0 => CPU_REFRESH_RATE / 1024,
            1 => CPU_REFRESH_RATE / 16,
            2 => CPU_REFRESH_RATE / 64,
            3 => CPU_REFRESH_RATE / 256,
            _ => panic!("Invalid input select value {}", self.input_clock_select())
        }
    }
}