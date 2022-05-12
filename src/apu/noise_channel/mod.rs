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

use super::{
    sound_length_wave_pattern::SoundLengthWavePattern, volume_envelope::VolumeEnvelope,
    LENGTH_COUNTER_MAX,
};

pub mod counter_consecutive_selection;
pub mod polynomial_counter;

#[derive(Serialize, Deserialize, Default)]
pub struct NoiseChannel {
    sound_length: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    polynomial_counter: PolynomialCounter,
    selection: CounterConsecutiveSelection,
    timer: u16,
    timer_load: u16,
    enabled: bool,
    dac_enabled: bool,
    output_volume: u8,
    volume: u8,
    length_counter: u8,
    lfsr: u16,

    //Envelope
    envelope_timer: u8,
    envelope_running: bool,
}

impl NoiseChannel {
    pub fn clock(&mut self) {
        self.timer -= 1;

        if self.timer == 0 {
            self.update_timer();

            let xor_result = (self.lfsr & 0x1) ^ ((self.lfsr >> 1) & 0x1);
            self.lfsr = (self.lfsr >> 1) | (xor_result << 14);
            if self.polynomial_counter.counter_step_width() {
                self.lfsr &= !(1 << 6);
                self.lfsr |= xor_result << 6;
            }

            self.output_volume = if self.enabled && (self.lfsr & 0x1) == 0 {
                self.volume
            } else {
                0
            };
        }
    }

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
        if !self.enabled {
            return;
        }

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
            CHANNEL_4_SOUND_LENGTH => {
                self.length_counter = self.sound_length.get_sound_length();
                self.sound_length.set_sound_length_data(value);
            }
            CHANNEL_4_VOLUME_ENVELOPE => {
                self.volume_envelope.0 = value;
            }
            CHANNEL_4_POLYNOMIAL_COUNTER => self.polynomial_counter.0 = value,
            CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL => {
                self.selection.0 = value;
                if self.selection.initialize() {
                    self.initialize();
                }
            }
            _ => invalid_address("APU", address),
        }
    }

    pub fn get_output_volume(&self) -> u8 {
        self.output_volume
    }

    pub fn is_on(&self) -> bool {
        self.length_counter > 0
    }

    pub fn reset_length_counter(&mut self) {
        self.length_counter = 0;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn initialize(&mut self) {
        self.enabled = true;
        self.lfsr = 0x7FFF;

        self.initialize_length_counter();
        self.initialize_envelope();
    }

    fn initialize_length_counter(&mut self) {
        if self.length_counter == 0 {
            self.length_counter = LENGTH_COUNTER_MAX;
        }
        self.update_timer();
    }

    fn initialize_envelope(&mut self) {
        self.envelope_running = true;
        self.envelope_timer = self.volume_envelope.initial_envelope_period();
        self.volume = self.volume_envelope.initial_volume();
    }

    fn update_timer(&mut self) {
        self.timer = self.polynomial_counter.get_frequency_timer();
    }
}
