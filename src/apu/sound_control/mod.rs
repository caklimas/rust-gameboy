use serde::{Deserialize, Serialize};

use crate::{
    addresses::apu::{CHANNEL_CONTROL, SOUND_CONTROL, SOUND_OUTPUT_TERMINAL_SELECTION},
    utils::invalid_address,
};

use self::{
    channel_control::ChannelControl, sound_output_terminal_selection::SoundOutputTerminalSelection,
    sound_trigger::SoundTrigger,
};

pub mod channel_control;
pub mod sound_output_terminal_selection;
pub mod sound_trigger;

#[derive(Serialize, Deserialize, Default)]
pub struct SoundControl {
    channel_control: ChannelControl,
    output_terminal_selection: SoundOutputTerminalSelection,
    sound_trigger: SoundTrigger,
}

impl SoundControl {
    pub fn read(&self, address: u16) -> u8 {
        match address {
            CHANNEL_CONTROL => self.channel_control.0,
            SOUND_OUTPUT_TERMINAL_SELECTION => self.output_terminal_selection.0,
            SOUND_CONTROL => self.sound_trigger.0,
            _ => invalid_address("APU", address),
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            CHANNEL_CONTROL => self.channel_control.0 = value,
            SOUND_OUTPUT_TERMINAL_SELECTION => self.output_terminal_selection.0 = value,
            SOUND_CONTROL => self
                .sound_trigger
                .set_all_sound_on_off(value & 0b1000_0000 != 0),
            _ => invalid_address("APU", address),
        }
    }
}
