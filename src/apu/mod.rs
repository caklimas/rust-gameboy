use serde::{Deserialize, Serialize};

pub mod channel_2;
pub mod sound_length_wave_pattern;

#[cfg(test)]
mod tests;

#[derive(Serialize, Deserialize, Default)]
pub struct Apu {
}

impl Apu {
}