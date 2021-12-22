use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_1_FREQUENCY_HI_DATA, CHANNEL_1_SWEEP_REGISTER, CHANNEL_2_FREQUENCY_HI_DATA,
    CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN, CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_SOUND_ON_OFF,
    CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL, CHANNEL_4_SOUND_LENGTH, CHANNEL_CONTROL, SOUND_CONTROL,
};

use self::{
    frame_sequencer::FrameSequencer, noise_channel::NoiseChannel, sound_control::SoundControl,
    square_channel::SquareChannel, wave_channel::WaveChannel,
};

pub mod frame_sequencer;
pub mod frequency_hi;
pub mod noise_channel;
pub mod sound_control;
pub mod sound_length_wave_pattern;
pub mod sound_on_off;
pub mod square_channel;
pub mod sweep_register;
pub mod volume_envelope;
pub mod wave_channel;

#[cfg(test)]
mod tests;

pub const LENGTH_COUNTER_MAX: u8 = 64;
pub const WAVE_DUTIES: [[bool; 8]; 4] = [
    [false, false, false, false, false, false, false, true], // 12.5%
    [true, false, false, false, false, false, false, true],  // 25%
    [true, false, false, false, false, true, true, true],    // 50%
    [false, true, true, true, true, true, true, false],      // 75%
];

const FRAME_SEQUENCE_COUNTDOWN_TICKS: u16 = 8192;
const FRAME_SEQUENCE_STEP_TICKS: u8 = 8;

#[derive(Serialize, Deserialize)]
pub struct Apu {
    channel_1: SquareChannel,
    channel_2: SquareChannel,
    channel_3: WaveChannel,
    channel_4: NoiseChannel,
    sound_control: SoundControl,
    wave_pattern_ram: [u8; 16],
    frame_sequencer: FrameSequencer,
}

impl Apu {
    pub fn clock(&mut self, cycles: u16) {
        // 1 CPU Cycle is 1 APU Cycle
        for _ in cycles..=0 {
            self.frame_sequencer.countdown -= 1;
            if self.frame_sequencer.countdown <= 0 {
                self.frame_sequencer.countdown = FRAME_SEQUENCE_COUNTDOWN_TICKS;

                match self.frame_sequencer.step {
                    0 => self.clock_length_counters(),
                    1 => (),
                    2 => {
                        self.clock_sweep();
                        self.clock_length_counters();
                    }
                    3 => (),
                    4 => self.clock_length_counters(),
                    5 => (), // Do nothing
                    6 => {
                        self.clock_sweep();
                        self.clock_length_counters();
                    }
                    7 => self.clock_volume_envelope(),
                    _ => panic!("Invalid frame sequencer step"),
                }

                self.frame_sequencer.step =
                    (self.frame_sequencer.step + 1) % FRAME_SEQUENCE_STEP_TICKS;
            }

            // Step all channels
            self.channel_1.step();
            self.channel_2.step();
            self.channel_3.step();
            self.channel_4.step();
        }
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

    fn clock_length_counters(&mut self) {
        self.channel_1.clock_length_counter();
        self.channel_2.clock_length_counter();
        self.channel_3.clock_length_counter();
        self.channel_4.clock_length_counter();
    }

    fn clock_sweep(&mut self) {
        self.channel_1.clock_sweep();
    }

    fn clock_volume_envelope(&mut self) {
        self.channel_1.clock_volume_envelope();
        self.channel_2.clock_volume_envelope();
        self.channel_4.clock_volume_envelope();
    }
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            channel_1: SquareChannel::with_sweep(),
            channel_2: Default::default(),
            channel_3: Default::default(),
            channel_4: Default::default(),
            sound_control: Default::default(),
            wave_pattern_ram: Default::default(),
            frame_sequencer: Default::default(),
        }
    }
}
