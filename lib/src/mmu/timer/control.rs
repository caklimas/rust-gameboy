use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct TimerControl(u8);
    impl Debug;

    pub input_clock_select, _: 1, 0;
    pub timer_enabled, _: 2;
    pub get, set: 2, 0;
}

impl TimerControl {
    pub fn get_step(&self) -> u32 {
        match self.input_clock_select() {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => panic!("Invalid input select value {}", self.input_clock_select()),
        }
    }
}
