use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_FREQUENCY_LO_DATA, CHANNEL_3_SELECT_OUTPUT_LEVEL,
    CHANNEL_3_SOUND_LENGTH, CHANNEL_3_SOUND_ON_OFF,
};

use super::{
    frequency_hi::FrequencyHi, sound_length_wave_pattern::SoundLengthWavePattern,
    sound_on_off::SoundOnOff, volume_envelope::VolumeEnvelope,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Channel3 {
    sound_on_off: SoundOnOff,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
}

impl Channel3 {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_3_SOUND_ON_OFF => self.sound_on_off.0,
            CHANNEL_3_SOUND_LENGTH => self.frequency_lo,
            CHANNEL_3_SELECT_OUTPUT_LEVEL => todo!(),
            CHANNEL_3_FREQUENCY_LO_DATA => self.frequency_lo,
            CHANNEL_3_FREQUENCY_HI_DATA => self.frequency_hi.0,
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_3_SOUND_ON_OFF => todo!(),
            CHANNEL_3_SOUND_LENGTH => todo!(),
            CHANNEL_3_SELECT_OUTPUT_LEVEL => todo!(),
            CHANNEL_3_FREQUENCY_LO_DATA => todo!(),
            CHANNEL_3_FREQUENCY_HI_DATA => todo!(),
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }
}
