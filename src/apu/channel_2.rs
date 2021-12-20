use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_2_FREQUENCY_HI_DATA, CHANNEL_2_FREQUENCY_LO_DATA, CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN,
    CHANNEL_2_VOLUME_ENVELOPE,
};

use super::{
    frequency_hi::FrequencyHi, sound_length_wave_pattern::SoundLengthWavePattern,
    volume_envelope::VolumeEnvelope,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Channel2 {
    sound_length_wave_pattern: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
}

impl Channel2 {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN => {
                self.sound_length_wave_pattern.get_sound_length()
            }
            CHANNEL_2_VOLUME_ENVELOPE => self.frequency_lo,
            CHANNEL_2_FREQUENCY_LO_DATA => self.frequency_lo,
            CHANNEL_2_FREQUENCY_HI_DATA => self.frequency_lo,
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN => self.sound_length_wave_pattern.0 = value,
            CHANNEL_2_VOLUME_ENVELOPE => self.volume_envelope.0 = value,
            CHANNEL_2_FREQUENCY_LO_DATA => self.frequency_lo = value,
            CHANNEL_2_FREQUENCY_HI_DATA => self.frequency_hi.0 = value,
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }
}
