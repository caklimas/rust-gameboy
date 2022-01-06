use serde::{Deserialize, Serialize};

use crate::{
    addresses::apu::{
        CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL, CHANNEL_4_POLYNOMIAL_COUNTER,
        CHANNEL_4_SOUND_LENGTH, CHANNEL_4_VOLUME_ENVELOPE,
    },
    constants::apu::{ENVELOPE_PERIOD_MAX, ENVELOPE_VOLUME_MAX, ENVELOPE_VOLUME_MIN},
    utils::invalid_address,
};

use self::{
    counter_consecutive_selection::CounterConsecutiveSelection,
    polynomial_counter::PolynomialCounter,
};

use super::{sound_length_wave_pattern::SoundLengthWavePattern, volume_envelope::VolumeEnvelope};

pub mod counter_consecutive_selection;
pub mod polynomial_counter;

#[derive(Serialize, Deserialize, Default)]
pub struct NoiseChannel {
    sound_length: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    polynomial_counter: PolynomialCounter,
    selection: CounterConsecutiveSelection,
    timer: u16,
    enabled: bool,
    output_volume: u8,
    volume: u8,
    length_counter: u8,

    //Envelope
    envelope_timer: u8,
    envelope_running: bool,
}

impl NoiseChannel {
    pub fn clock(&mut self) {}

    pub fn clock_length_counter(&mut self) {
        if self.length_counter == 0 || !self.selection.length_enabled() {
            return;
        }

        self.length_counter -= 1;
        if self.length_counter == 0 {
            self.enabled = false;
        }
    }

    pub fn clock_volume_envelope(&mut self) {
        self.envelope_timer -= 1;
        if self.envelope_timer > 0 {
            return;
        }

        self.envelope_timer = self.volume_envelope.initial_envelope_period();
        if self.envelope_timer == 0 {
            self.envelope_timer = ENVELOPE_PERIOD_MAX;
        }

        if self.envelope_running && self.volume_envelope.initial_envelope_period() > 0 {
            let direction = self.volume_envelope.direction();
            if direction && self.volume < ENVELOPE_VOLUME_MAX {
                self.volume += 1;
            } else if !direction && self.volume > ENVELOPE_VOLUME_MIN {
                self.volume -= 1;
            }
        }

        if self.volume == ENVELOPE_VOLUME_MIN || self.volume == ENVELOPE_VOLUME_MAX {
            self.envelope_running = false;
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_4_SOUND_LENGTH => panic!("Channel 4 sound length is write only"),
            CHANNEL_4_VOLUME_ENVELOPE => self.volume_envelope.0,
            CHANNEL_4_POLYNOMIAL_COUNTER => self.polynomial_counter.0,
            CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL => self.selection.0,
            _ => invalid_address("APU", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_4_SOUND_LENGTH => self.sound_length.set_sound_length_data(value),
            CHANNEL_4_VOLUME_ENVELOPE => self.volume_envelope.0 = value,
            CHANNEL_4_POLYNOMIAL_COUNTER => self.polynomial_counter.0 = value,
            CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL => self.selection.0 = value,
            _ => invalid_address("APU", address),
        }
    }
}
