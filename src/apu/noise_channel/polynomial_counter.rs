use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct PolynomialCounter(u8);
    impl Debug;

    pub shift_clock_frequency, _: 7, 4;
    pub counter_step_width, _: 3;
    pub frequency_dividing_ratio, _: 2, 0;
}

impl PolynomialCounter {
    pub fn get_frequency_timer(&self) -> u16 {
        self.get_divisor() << self.shift_clock_frequency()
    }

    fn get_divisor(&self) -> u16 {
        match self.frequency_dividing_ratio() {
            0 => 8,
            1 => 16,
            2 => 32,
            3 => 48,
            4 => 64,
            5 => 80,
            6 => 96,
            7 => 112,
            _ => panic!("Invalid dividing ratio"),
        }
    }
}
