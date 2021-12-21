use serde::{Deserialize, Serialize};

use crate::{
    addresses::apu::{
        CHANNEL_1_FREQUENCY_HI_DATA, CHANNEL_1_FREQUENCY_LO_DATA,
        CHANNEL_1_SOUND_LENGTH_WAVE_PATTERN, CHANNEL_1_SWEEP_REGISTER, CHANNEL_1_VOLUME_ENVELOPE,
        CHANNEL_2_FREQUENCY_HI_DATA, CHANNEL_2_FREQUENCY_LO_DATA,
        CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN, CHANNEL_2_VOLUME_ENVELOPE,
    },
    utils::invalid_address,
};

use super::{
    frequency_hi::FrequencyHi, sound_length_wave_pattern::SoundLengthWavePattern,
    sweep_register::SweepRegister, volume_envelope::VolumeEnvelope,
};

#[derive(Serialize, Deserialize, Default)]
pub struct SquareChannel {
    sweep_register: Option<SweepRegister>,
    sound_length_wave_pattern: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
    sequence_pointer: usize,
    timer: u16,
}

impl SquareChannel {
    pub fn clock(&mut self, cycles: u16) {
        self.timer -= cycles;
        if self.timer <= 0 {
            self.sequence_pointer = (self.sequence_pointer + 1) % 8;
            self.update_timer();
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_1_SWEEP_REGISTER => self.get_sweep_register().0,
            CHANNEL_1_SOUND_LENGTH_WAVE_PATTERN | CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN => {
                self.sound_length_wave_pattern.get_sound_length()
            }
            CHANNEL_1_VOLUME_ENVELOPE | CHANNEL_2_VOLUME_ENVELOPE => self.frequency_lo,
            CHANNEL_1_FREQUENCY_LO_DATA | CHANNEL_2_FREQUENCY_LO_DATA => self.frequency_lo,
            CHANNEL_1_FREQUENCY_HI_DATA | CHANNEL_2_FREQUENCY_HI_DATA => self.frequency_lo,
            _ => invalid_address("APU", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_1_SWEEP_REGISTER => self.set_sweep_register(value),
            CHANNEL_1_SOUND_LENGTH_WAVE_PATTERN | CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN => {
                self.sound_length_wave_pattern.0 = value
            }
            CHANNEL_1_VOLUME_ENVELOPE | CHANNEL_2_VOLUME_ENVELOPE => self.volume_envelope.0 = value,
            CHANNEL_1_FREQUENCY_LO_DATA | CHANNEL_2_FREQUENCY_LO_DATA => {
                self.frequency_lo = value;
                self.update_timer();
            }
            CHANNEL_1_FREQUENCY_HI_DATA | CHANNEL_2_FREQUENCY_HI_DATA => {
                self.frequency_hi.0 = value;
                self.update_timer();
            }
            _ => invalid_address("APU", address),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }

    pub fn get_output(&self) -> u8 {
        if self.sound_length_wave_pattern.get_wave_duty()[self.sequence_pointer] {
            1
        } else {
            0
        }
    }

    fn update_timer(&mut self) {
        let frequency = self.get_frequency();
        self.timer = if frequency > 2048 {
            0
        } else {
            (2048 - frequency) * 4
        }
    }

    fn get_sweep_register(&self) -> &SweepRegister {
        self.sweep_register
            .as_ref()
            .unwrap_or_else(|| panic!("Sweep not available"))
    }

    fn set_sweep_register(&mut self, value: u8) {
        let mut sweep_register = self
            .sweep_register
            .as_mut()
            .unwrap_or_else(|| panic!("Sweep not available"));
        sweep_register.0 = value
    }
}
