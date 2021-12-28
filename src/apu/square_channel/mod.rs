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
    frequency_hi::FrequencyHi,
    sound_length_wave_pattern::SoundLengthWavePattern,
    sweep_register::{SweepRegister, SWEEP_PERIOD_MAX},
    volume_envelope::{
        VolumeEnvelope, ENVELOPE_PERIOD_MAX, ENVELOPE_VOLUME_MAX, ENVELOPE_VOLUME_MIN,
    },
    LENGTH_COUNTER_MAX,
};

const FREQUENCY_MAX: u16 = 2047;

#[derive(Serialize, Deserialize, Default)]
pub struct SquareChannel {
    sweep_register: Option<SweepRegister>,
    sound_length_wave_pattern: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    frequency_lo: u8,
    frequency_hi: FrequencyHi,
    sequence_pointer: usize,
    timer: u16,
    frequency: u16,
    enabled: bool,
    output_volume: u8,
    volume: u8,
    length_counter: u8,
    envelope_timer: u8,
    envelope_running: bool,
    sweep_period: u8,
    sweep_enabled: bool,
    shadow_frequency: u16,
}

impl SquareChannel {
    pub fn with_sweep() -> Self {
        SquareChannel {
            sweep_register: Option::Some(Default::default()),
            ..Default::default()
        }
    }

    pub fn clock(&mut self) {
        self.timer -= 1;
        if self.timer == 0 {
            self.sequence_pointer = (self.sequence_pointer + 1) % 8;
            self.update_timer();
        }

        self.output_volume = if self.enabled { self.volume } else { 0 };
    }

    pub fn clock_length_counter(&mut self) {
        if self.length_counter == 0 || !self.frequency_hi.length_enabled() {
            return;
        }

        self.length_counter -= 1;
        if self.length_counter == 0 {
            self.enabled = false;
        }
    }

    pub fn clock_sweep(&mut self) {
        if self.sweep_register.is_none() {
            return;
        }

        self.sweep_period -= 1;
        if self.sweep_period > 0 {
            return;
        }

        let sweep = self.sweep_register.as_ref().unwrap();
        self.sweep_period = sweep.time();
        if self.sweep_period == 0 {
            self.sweep_period = SWEEP_PERIOD_MAX;
        }

        if self.sweep_enabled && sweep.time() > 0 {
            let (new_frequency, disable) = self.calculate_sweep();
            if new_frequency <= FREQUENCY_MAX && sweep.shift() > 0 {
                self.shadow_frequency = new_frequency;
                self.frequency = new_frequency;
                self.calculate_sweep();
            }

            if disable {
                self.enabled = false;
            }

            self.calculate_sweep();
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
                self.sound_length_wave_pattern.0 = value;
                self.length_counter = self.sound_length_wave_pattern.get_sound_length();
            }
            CHANNEL_1_VOLUME_ENVELOPE | CHANNEL_2_VOLUME_ENVELOPE => {
                self.volume_envelope.0 = value;
                self.envelope_timer = self.volume_envelope.initial_envelope_period();
                self.volume = self.volume_envelope.initial_volume();
            }
            CHANNEL_1_FREQUENCY_LO_DATA | CHANNEL_2_FREQUENCY_LO_DATA => {
                self.frequency_lo = value;
                self.frequency = self.get_frequency();
            }
            CHANNEL_1_FREQUENCY_HI_DATA | CHANNEL_2_FREQUENCY_HI_DATA => {
                self.frequency_hi.0 = value;
                self.frequency = self.get_frequency();
                if self.frequency_hi.initialize() {
                    self.initialize();
                }
            }
            _ => invalid_address("APU", address),
        }
    }

    pub fn get_frequency(&self) -> u16 {
        let hi = (self.frequency_hi.frequency_higher_bits() as u16) << 8;
        let lo = self.frequency_lo as u16;
        hi | lo
    }

    pub fn get_output_volume(&self) -> u8 {
        if self.sound_length_wave_pattern.get_wave_duty()[self.sequence_pointer] {
            self.output_volume
        } else {
            0
        }
    }

    fn update_timer(&mut self) {
        self.timer = if self.frequency > 2048 {
            0
        } else {
            (2048 - self.frequency) * 4
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

    fn initialize(&mut self) {
        self.enabled = true;

        self.initialize_length_counter();
        self.initialize_envelope();
        self.initialize_sweep();

        self.shadow_frequency = self.get_frequency();
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

    fn initialize_sweep(&mut self) {
        if self.sweep_register.is_none() {
            return;
        }

        let sweep = self.sweep_register.as_ref().unwrap();
        self.shadow_frequency = self.get_frequency();
        self.sweep_period = sweep.time();
        if self.sweep_period == 0 {
            self.sweep_period = SWEEP_PERIOD_MAX;
        }

        self.sweep_enabled = self.sweep_period > 0 || sweep.shift() > 0;
        if sweep.shift() > 0 {
            self.calculate_sweep();
        }
    }

    fn calculate_sweep(&self) -> (u16, bool) {
        let sweep = self.sweep_register.as_ref().unwrap();
        let mut new_frequency = self.shadow_frequency >> sweep.shift();
        if sweep.decrease() {
            new_frequency = self.shadow_frequency - new_frequency;
        } else {
            new_frequency += self.shadow_frequency;
        }

        (new_frequency, new_frequency > FREQUENCY_MAX)
    }
}
