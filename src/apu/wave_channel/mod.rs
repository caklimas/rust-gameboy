use serde::{Deserialize, Serialize};

use crate::{
    addresses::apu::{
        CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_FREQUENCY_LO_DATA, CHANNEL_3_SELECT_OUTPUT_LEVEL,
        CHANNEL_3_SOUND_LENGTH, CHANNEL_3_SOUND_ON_OFF,
    },
    utils::invalid_address,
};

use self::select_output_level::SelectOutputLevel;

use super::{frequency_hi::FrequencyHi, sound_on_off::SoundOnOff};

pub mod select_output_level;

#[derive(Serialize, Deserialize, Default)]
pub struct WaveChannel {
    sound_on_off: SoundOnOff,
    sound_length: u8,
    select_output_level: SelectOutputLevel,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
}

impl WaveChannel {
    pub fn step(&mut self) {}

    pub fn clock_length_counter(&mut self) {}

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_3_SOUND_ON_OFF => self.sound_on_off.0,
            CHANNEL_3_SOUND_LENGTH => panic!("Can't read address 0x{:4X}", address),
            CHANNEL_3_SELECT_OUTPUT_LEVEL => self.select_output_level.0,
            CHANNEL_3_FREQUENCY_LO_DATA => panic!("Can't read address 0x{:4X}", address),
            CHANNEL_3_FREQUENCY_HI_DATA => self.frequency_hi.0,
            _ => invalid_address("APU", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_3_SOUND_ON_OFF => self.sound_on_off.0 = value,
            CHANNEL_3_SOUND_LENGTH => self.sound_length = value,
            CHANNEL_3_SELECT_OUTPUT_LEVEL => self.select_output_level.0 = value,
            CHANNEL_3_FREQUENCY_LO_DATA => self.frequency_lo = value,
            CHANNEL_3_FREQUENCY_HI_DATA => self.frequency_hi.0 = value,
            _ => invalid_address("APU", address),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }
}
