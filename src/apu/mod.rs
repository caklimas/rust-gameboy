use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_1_FREQUENCY_HI_DATA, CHANNEL_1_SWEEP_REGISTER, CHANNEL_2_FREQUENCY_HI_DATA,
    CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN, CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_SOUND_ON_OFF,
};

use self::{channel_1::Channel1, channel_2::Channel2, channel_3::Channel3};

pub mod channel_1;
pub mod channel_2;
pub mod channel_3;
pub mod frequency_hi;
pub mod select_output_level;
pub mod sound_length_wave_pattern;
pub mod sound_on_off;
pub mod sweep_register;
pub mod volume_envelope;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Apu {
    channel_1: Channel1,
    channel_2: Channel2,
    channel_3: Channel3,
    wave_pattern_ram: [u8; 16],
}

impl Apu {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_1_SWEEP_REGISTER..=CHANNEL_1_FREQUENCY_HI_DATA => self.channel_1.read(address),
            CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN..=CHANNEL_2_FREQUENCY_HI_DATA => {
                self.channel_2.read(address)
            }
            CHANNEL_3_SOUND_ON_OFF..=CHANNEL_3_FREQUENCY_HI_DATA => self.channel_3.read(address),
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_1_SWEEP_REGISTER..=CHANNEL_1_FREQUENCY_HI_DATA => {
                self.channel_1.write(address, value)
            }
            CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN..=CHANNEL_2_FREQUENCY_HI_DATA => {
                self.channel_2.write(address, value)
            }
            CHANNEL_3_SOUND_ON_OFF..=CHANNEL_3_FREQUENCY_HI_DATA => {
                self.channel_3.write(address, value)
            }
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }
}
