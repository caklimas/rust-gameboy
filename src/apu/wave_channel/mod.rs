use serde::{Deserialize, Serialize};

use crate::{
    addresses::apu::{
        CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_FREQUENCY_LO_DATA, CHANNEL_3_SELECT_OUTPUT_LEVEL,
        CHANNEL_3_SOUND_LENGTH, CHANNEL_3_SOUND_ON_OFF, WAVE_PATTERN_RAM_LOWER,
        WAVE_PATTERN_RAM_UPPER,
    },
    utils::invalid_address,
};

use self::{select_output_level::SelectOutputLevel, wave_ram::WaveRam};

use super::{frequency_hi::FrequencyHi, sound_on_off::SoundOnOff};

pub mod select_output_level;
pub mod wave_ram;

#[derive(Serialize, Deserialize, Default)]
pub struct WaveChannel {
    sound_on_off: SoundOnOff,
    sound_length: u8,
    select_output_level: SelectOutputLevel,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
    length_counter: u8,
    wave_ram: WaveRam,
    enabled: bool,
    timer: u16,
    position_counter: u8,
    output_volume: u8,
}

impl WaveChannel {
    pub fn clock(&mut self) {
        self.timer -= 1;
        if self.timer > 0 {
            return;
        }

        self.timer = self.calculate_timer();
        self.output_volume = if self.enabled {
            let position = self.position_counter / 2;
            let mut output = self.wave_ram.read(position as usize);
            if self.position_counter & 0b1 > 0 {
                output >>= 4;
            }
            output &= 0xF;

            if self.select_output_level.0 > 0 {
                output >>= self.select_output_level.0 - 1;
            } else {
                output = 0;
            }

            output
        } else {
            0
        };
    }

    pub fn clock_length_counter(&mut self) {}

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_3_SOUND_ON_OFF => self.sound_on_off.0,
            CHANNEL_3_SOUND_LENGTH => panic!("Can't read address 0x{:4X}", address),
            CHANNEL_3_SELECT_OUTPUT_LEVEL => self.select_output_level.0,
            CHANNEL_3_FREQUENCY_LO_DATA => panic!("Can't read address 0x{:4X}", address),
            CHANNEL_3_FREQUENCY_HI_DATA => self.frequency_hi.0,
            WAVE_PATTERN_RAM_LOWER..=WAVE_PATTERN_RAM_UPPER => {
                self.wave_ram.read(self.get_offset(address))
            }
            _ => invalid_address("APU", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_3_SOUND_ON_OFF => self.sound_on_off.0 = value,
            CHANNEL_3_SOUND_LENGTH => self.sound_length = value,
            CHANNEL_3_SELECT_OUTPUT_LEVEL => self.select_output_level.0 = value,
            CHANNEL_3_FREQUENCY_LO_DATA => self.frequency_lo = value,
            CHANNEL_3_FREQUENCY_HI_DATA => {
                self.frequency_hi.0 = value;
                if self.frequency_hi.initialize() {
                    self.trigger();
                }
            }
            WAVE_PATTERN_RAM_LOWER..=WAVE_PATTERN_RAM_UPPER => {
                self.wave_ram.write(self.get_offset(address), value)
            }
            _ => invalid_address("APU", address),
        }
    }

    pub fn is_on(&self) -> bool {
        self.length_counter > 0
    }

    pub fn reset_length_counter(&mut self) {
        self.length_counter = 0;
    }

    fn trigger(&mut self) {
        self.enabled = true;
        if self.length_counter == 0 {
            self.length_counter = 255;
        }

        self.timer = self.calculate_timer();
        self.position_counter = 0;
    }

    fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }

    fn calculate_timer(&self) -> u16 {
        (2048 - self.get_frequency()) * 2
    }

    fn get_offset(&self, address: u16) -> usize {
        (address - WAVE_PATTERN_RAM_LOWER) as usize
    }
}
