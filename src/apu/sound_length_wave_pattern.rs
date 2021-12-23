use serde::{Deserialize, Serialize};

use super::WAVE_DUTIES;

bitfield! {
    #[derive(Serialize, Deserialize, Default)]
    pub struct SoundLengthWavePattern(u8);
    impl Debug;

    pub wave_pattern_duty, _: 7, 6;
    pub sound_length_data, set_sound_length_data: 5, 0;
}

impl SoundLengthWavePattern {
    pub fn get_wave_duty(&self) -> [bool; 8] {
        let duty = self.wave_pattern_duty() as usize;
        match duty {
            0..=3 => WAVE_DUTIES[duty],
            _ => panic!("Invalid wave_pattern_duty"),
        }
    }

    pub fn get_sound_length(&self) -> u8 {
        64 - self.sound_length_data()
    }
}
