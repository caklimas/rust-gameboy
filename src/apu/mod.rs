use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_1_FREQUENCY_HI_DATA, CHANNEL_1_SWEEP_REGISTER, CHANNEL_2_FREQUENCY_HI_DATA,
    CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN, CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_SOUND_ON_OFF,
    CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL, CHANNEL_4_SOUND_LENGTH, CHANNEL_CONTROL, SOUND_CONTROL,
};

use self::{
    channel_3::Channel3, channel_4::Channel4, sound_control::SoundControl,
    square_channel::SquareChannel,
};

pub mod channel_3;
pub mod channel_4;
pub mod frequency_hi;
pub mod sound_control;
pub mod sound_length_wave_pattern;
pub mod sound_on_off;
pub mod square_channel;
pub mod sweep_register;
pub mod volume_envelope;

#[cfg(test)]
mod tests;

pub const WAVE_DUTIES: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true], // 12.5%
    [true, false, false, false, false, false, false, true],  // 25%
    [true, false, false, false, false, true, true, true],    // 50%
    [false, true, true, true, true, true, true, false],      // 75%
];

#[derive(Serialize, Deserialize, Default)]
pub struct Apu {
    channel_1: SquareChannel,
    channel_2: SquareChannel,
    channel_3: Channel3,
    channel_4: Channel4,
    sound_control: SoundControl,
    wave_pattern_ram: [u8; 16],
}

impl Apu {
    pub fn clock(&mut self, cycles: u16) {
        self.channel_1.clock(cycles);
        self.channel_2.clock(cycles);
        self.channel_3.clock(cycles);
        self.channel_4.clock(cycles);
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_1_SWEEP_REGISTER..=CHANNEL_1_FREQUENCY_HI_DATA => self.channel_1.read(address),
            CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN..=CHANNEL_2_FREQUENCY_HI_DATA => {
                self.channel_2.read(address)
            }
            CHANNEL_3_SOUND_ON_OFF..=CHANNEL_3_FREQUENCY_HI_DATA => self.channel_3.read(address),
            CHANNEL_4_SOUND_LENGTH..=CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL => {
                self.channel_4.read(address)
            }
            CHANNEL_CONTROL..=SOUND_CONTROL => self.sound_control.read(address),
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
            CHANNEL_4_SOUND_LENGTH..=CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL => {
                self.channel_4.write(address, value)
            }
            CHANNEL_CONTROL..=SOUND_CONTROL => self.sound_control.write(address, value),
            _ => panic!("Invalid APU address 0x{:4X}", address),
        }
    }
}
