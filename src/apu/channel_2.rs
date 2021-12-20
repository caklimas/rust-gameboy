use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_2_FREQUENCY_HI_DATA, CHANNEL_2_FREQUENCY_LO_DATA, CHANNEL_2_SOUND_LENGTH,
    CHANNEL_2_VOLUME_ENVELOPE,
};

use super::sound_length_wave_pattern::SoundLengthWavePattern;

#[derive(Serialize, Deserialize, Default)]
pub struct Channel2 {
    sound_length_wave_pattern: SoundLengthWavePattern,
    frequency_lo: u8,
}

impl Channel2 {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_2_SOUND_LENGTH => self.sound_length_wave_pattern.get_sound_length(),
            CHANNEL_2_VOLUME_ENVELOPE => self.frequency_lo,
            CHANNEL_2_FREQUENCY_LO_DATA => self.frequency_lo,
            CHANNEL_2_FREQUENCY_HI_DATA => self.frequency_lo,
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }
}
