use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_1_FREQUENCY_HI_DATA, CHANNEL_1_FREQUENCY_LO_DATA, CHANNEL_1_SOUND_LENGTH_WAVE_PATTERN,
    CHANNEL_1_SWEEP_REGISTER, CHANNEL_1_VOLUME_ENVELOPE,
};

use super::{
    frequency_hi::FrequencyHi, sound_length_wave_pattern::SoundLengthWavePattern,
    sweep_register::SweepRegister, volume_envelope::VolumeEnvelope,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Channel1 {
    sweep_register: SweepRegister,
    sound_length_wave_pattern: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
}

impl Channel1 {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_1_SWEEP_REGISTER => self.sweep_register.0,
            CHANNEL_1_SOUND_LENGTH_WAVE_PATTERN => {
                self.sound_length_wave_pattern.get_sound_length()
            }
            CHANNEL_1_VOLUME_ENVELOPE => self.frequency_lo,
            CHANNEL_1_FREQUENCY_LO_DATA => self.frequency_lo,
            CHANNEL_1_FREQUENCY_HI_DATA => self.frequency_hi.0,
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_1_SWEEP_REGISTER => self.sweep_register.0 = value,
            CHANNEL_1_SOUND_LENGTH_WAVE_PATTERN => self.sound_length_wave_pattern.0 = value,
            CHANNEL_1_VOLUME_ENVELOPE => self.volume_envelope.0 = value,
            CHANNEL_1_FREQUENCY_LO_DATA => self.frequency_lo = value,
            CHANNEL_1_FREQUENCY_HI_DATA => self.frequency_hi.0 = value,
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }
}
