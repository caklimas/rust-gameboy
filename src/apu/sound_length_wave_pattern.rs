use serde::{Serialize, Deserialize};

bitfield!{
    #[derive(Serialize, Deserialize, Default)]
    pub struct SoundLengthWavePattern(u8);
    impl Debug;

    pub wave_pattern_duty, set_wave_pattern_duty: 7, 6;
    pub sound_length_data, set_sound_length_data: 5, 0;
}

impl SoundLengthWavePattern {
    pub fn get_ratio(&self) -> f64 {
        match self.wave_pattern_duty() {
            0 => 0.125,
            1 => 0.25,
            2 => 0.50,
            3 => 0.75,
            _ => panic!("Invalid wave_pattern_duty")
        }
    }

    pub fn get_sound_length(&self) -> u8 {
        64 - self.sound_length_data()
    }
}