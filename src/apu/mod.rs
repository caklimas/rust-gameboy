use serde::{Deserialize, Serialize};

use crate::addresses::apu::{CHANNEL_2_FREQUENCY_HI_DATA, CHANNEL_2_SOUND_LENGTH};

use self::channel_2::Channel2;

pub mod channel_2;
pub mod sound_length_wave_pattern;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Apu {
    channel_2: Channel2,
}

impl Apu {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_2_SOUND_LENGTH..=CHANNEL_2_FREQUENCY_HI_DATA => self.channel_2.read(address),
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }
}
