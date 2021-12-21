use serde::{Deserialize, Serialize};

use super::{
    counter_consecutive_selection::CounterConsecutiveSelection,
    polynomial_counter::PolynomialCounter, sound_length_wave_pattern::SoundLengthWavePattern,
    volume_envelope::VolumeEnvelope,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Channel4 {
    sound_length: SoundLengthWavePattern,
    volume_envelope: VolumeEnvelope,
    polynomial_counter: PolynomialCounter,
    selection: CounterConsecutiveSelection,
}
