use serde::{Deserialize, Serialize};

use crate::{
    addresses::apu::{
        CHANNEL_4_COUNTER_CONSECUTIVE_INITIAL, CHANNEL_4_POLYNOMIAL_COUNTER,
        CHANNEL_4_SOUND_LENGTH, CHANNEL_4_VOLUME_ENVELOPE,
    },
    utils::invalid_address,
};

use self::{
    counter_consecutive_selection::CounterConsecutiveSelection,
    polynomial_counter::PolynomialCounter,
};

use super::{sound_length_wave_pattern::SoundLengthWavePattern, volume_envelope::VolumeEnvelope};

pub mod counter_consecutive_selection;
pub mod polynomial_counter;

/** Noise Channel
 * Volume Envelope
*/
#[derive(Serialize, Deserialize, Default)]
pub struct Channel4 {
    sound_length: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    polynomial_counter: PolynomialCounter,
    selection: CounterConsecutiveSelection,
}

impl Channel4 {
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
