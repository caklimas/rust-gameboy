use serde::{Deserialize, Serialize};

use crate::addresses::apu::{
    CHANNEL_1_FREQUENCY_HI_DATA, CHANNEL_1_SWEEP_REGISTER, CHANNEL_2_FREQUENCY_HI_DATA,
    CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN, CHANNEL_3_FREQUENCY_HI_DATA, CHANNEL_3_SOUND_ON_OFF,
    CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL, CHANNEL_4_SOUND_LENGTH, CHANNEL_CONTROL, SOUND_CONTROL,
    SOUND_OUTPUT_TERMINAL_SELECTION, WAVE_PATTERN_RAM_LOWER, WAVE_PATTERN_RAM_UPPER,
};

use crate::constants::apu::SAMPLE_SIZE;

use self::{
    audio_formatter::mix_audio, frame_sequencer::FrameSequencer, noise_channel::NoiseChannel,
    sound_control::SoundControl, square_channel::SquareChannel, wave_channel::WaveChannel,
};

pub mod audio_formatter;
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
const SAMPLE_COUNTER_MAX: u8 = 95;

#[derive(Serialize, Deserialize)]
pub struct Apu {
    channel_1: SquareChannel,
    channel_2: SquareChannel,
    channel_3: WaveChannel,
    channel_4: NoiseChannel,
    sound_control: SoundControl,
    wave_pattern_ram: [u8; 16],
    frame_sequencer: FrameSequencer,
    sample_counter: u8,

    #[serde(skip_serializing, skip_deserializing, default = "get_default_buffer")]
    buffer: [f32; SAMPLE_SIZE],
    buffer_index: usize,
}

impl Apu {
    pub fn clock(&mut self, cycles: u16) -> bool {
        let mut audio_buffer_full = false;

        // 1 CPU Cycle is 1 APU Cycle
        for _ in 0..cycles {
            self.frame_sequencer.countdown = self.frame_sequencer.countdown.saturating_sub(1);
            if self.frame_sequencer.countdown == 0 {
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
            self.channel_1.clock();
            self.channel_2.clock();
            self.channel_3.clock();
            self.channel_4.clock();

            self.sample_counter = self.sample_counter.saturating_sub(1);
            if self.sample_counter == 0 {
                self.sample_counter = SAMPLE_COUNTER_MAX;

                self.buffer_left_channel();
                self.buffer_right_channel();
            }

            if self.buffer_index >= SAMPLE_SIZE {
                self.buffer_index = 0;
                audio_buffer_full = true;
                break;
            }
        }

        audio_buffer_full
    }

    pub fn get_audio_buffer(&self) -> &[f32] {
        &self.buffer
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_1_SWEEP_REGISTER..=CHANNEL_1_FREQUENCY_HI_DATA => self.channel_1.read(address),
            0xFF15 => 0xFF,
            CHANNEL_2_SOUND_LENGTH_WAVE_PATTERN..=CHANNEL_2_FREQUENCY_HI_DATA => {
                self.channel_2.read(address)
            }
            CHANNEL_3_SOUND_ON_OFF..=CHANNEL_3_FREQUENCY_HI_DATA => self.channel_3.read(address),
            CHANNEL_4_SOUND_LENGTH..=CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL => {
                self.channel_4.read(address)
            }
            CHANNEL_CONTROL..=SOUND_OUTPUT_TERMINAL_SELECTION => self.sound_control.read(address),
            SOUND_CONTROL => {
                let mut value: u8 = 0;
                value |= if self.sound_control.sound_trigger.all_sound_on_off() {
                    1
                } else {
                    0
                } << 7;

                value |= if self.channel_4.is_on() { 1 } else { 0 } << 3;
                value |= if self.channel_3.is_on() { 1 } else { 0 } << 2;
                value |= if self.channel_2.is_on() { 1 } else { 0 } << 1;
                value |= if self.channel_1.is_on() { 1 } else { 0 };

                value
            }
            WAVE_PATTERN_RAM_LOWER..=WAVE_PATTERN_RAM_UPPER => self.channel_3.read(address),
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
            CHANNEL_CONTROL..=SOUND_OUTPUT_TERMINAL_SELECTION => {
                self.sound_control.write(address, value)
            }
            SOUND_CONTROL => {
                self.sound_control
                    .sound_trigger
                    .set_all_sound_on_off(value & 0b1000_0000 != 0);

                if !self.sound_control.sound_trigger.all_sound_on_off() {
                    self.channel_1.reset_length_counter();
                    self.channel_2.reset_length_counter();
                    self.channel_3.reset_length_counter();
                    self.channel_4.reset_length_counter();
                }
            }
            WAVE_PATTERN_RAM_LOWER..=WAVE_PATTERN_RAM_UPPER => self.channel_3.write(address, value),
            _ => (),
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

    fn buffer_left_channel(&mut self) {
        let mut buffer_input = 0.0_f32;
        let volume = (self.sound_control.channel_control.s02_output_level() as u16 * 128) / 7;

        if self
            .sound_control
            .output_terminal_selection
            .sound_1_to_s02()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_1.get_output_volume() as f32 / 100.0,
                volume,
            );
        }
        if self
            .sound_control
            .output_terminal_selection
            .sound_2_to_s02()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_2.get_output_volume() as f32 / 100.0,
                volume,
            );
        }
        if self
            .sound_control
            .output_terminal_selection
            .sound_3_to_s02()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_3.get_output_volume() as f32 / 100.0,
                volume,
            );
        }
        if self
            .sound_control
            .output_terminal_selection
            .sound_4_to_s02()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_4.get_output_volume() as f32 / 100.0,
                volume,
            );
        }

        self.buffer[self.buffer_index] = buffer_input;
        self.buffer_index += 1;
    }

    fn buffer_right_channel(&mut self) {
        let mut buffer_input = 0.0;
        let volume = (self.sound_control.channel_control.s01_output_level() as u16 * 128) / 7;

        if self
            .sound_control
            .output_terminal_selection
            .sound_1_to_s01()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_1.get_output_volume() as f32 / 100.0,
                volume,
            );
        }
        if self
            .sound_control
            .output_terminal_selection
            .sound_2_to_s01()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_2.get_output_volume() as f32 / 100.0,
                volume,
            );
        }
        if self
            .sound_control
            .output_terminal_selection
            .sound_3_to_s01()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_3.get_output_volume() as f32 / 100.0,
                volume,
            );
        }
        if self
            .sound_control
            .output_terminal_selection
            .sound_4_to_s01()
        {
            buffer_input = mix_audio(
                buffer_input,
                self.channel_4.get_output_volume() as f32 / 100.0,
                volume,
            );
        }

        self.buffer[self.buffer_index] = buffer_input;
        self.buffer_index += 1;
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
            sample_counter: Default::default(),
            buffer: get_default_buffer(),
            buffer_index: 0,
        }
    }
}

fn get_default_buffer() -> [f32; SAMPLE_SIZE] {
    [Default::default(); SAMPLE_SIZE]
}
