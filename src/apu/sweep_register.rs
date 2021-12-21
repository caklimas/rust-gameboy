use serde::{Deserialize, Serialize};

bitfield! {
    /// Sweep is a way to adjust the frequency of a channel periodically.
    #[derive(Serialize, Deserialize, Default)]
    pub struct SweepRegister(u8);
    impl Debug;

    pub time, _: 6, 4;
    pub direction, _: 3; // Sweep Increase/Decrease 0: Addition    (frequency increases) 1: Subtraction (frequency decreases)
    pub shift, _: 2, 0; // Number of sweep shift (n: 0-7)
}

impl SweepRegister {
    pub fn get_sweep_time(&self) -> f64 {
        let hz = 128.0;
        let numerator = match self.time() {
            0b000 => 0.0,
            0b001 => 1.0,
            0b010 => 2.0,
            0b011 => 3.0,
            0b100 => 4.0,
            0b101 => 5.0,
            0b110 => 6.0,
            0b111 => 7.0,
            _ => panic!("Invalid sweep time value"),
        };

        numerator / hz
    }
}
