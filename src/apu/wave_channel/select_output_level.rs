use serde::{Deserialize, Serialize};

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SelectOutputLevel(u8);
    impl Debug;

    pub select_output_level, _: 6, 5;
}

impl SelectOutputLevel {
    pub fn get_output_level(&self) -> f64 {
        match self.select_output_level() {
            0b00 => 0.0,
            0b01 => 1.0,
            0b10 => 0.5,
            0b11 => 0.25,
            _ => panic!("Invalid output level"),
        }
    }
}
