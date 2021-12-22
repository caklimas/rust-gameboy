use serde::{Deserialize, Serialize};

pub const SWEEP_PERIOD_MAX: u8 = 8;

bitfield! {
    /// Sweep is a way to adjust the frequency of a channel periodically.
    #[derive(Serialize, Deserialize, Default)]
    pub struct SweepRegister(u8);
    impl Debug;

    pub time, _: 6, 4;
    pub decrease, _: 3; // Sweep Increase/Decrease 0: Addition    (frequency increases) 1: Subtraction (frequency decreases)
    pub shift, _: 2, 0; // Number of sweep shift (n: 0-7)
}

impl SweepRegister {
    pub fn get_sweep_time(&self) -> f64 {
        let hz = 128.0;
        let time = self.time();
        let numerator = match self.time() {
            0..=7 => time as f64,
            _ => panic!("Invalid sweep time value"),
        };

        numerator / hz
    }
}
