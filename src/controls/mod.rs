use serde::{Serialize, Deserialize};
use crate::input::*;

#[cfg(test)]
mod tests;

pub const SELECT_BUTTON_BIT: u8 = 1 << 5;
pub const SELECT_DIRECTION_BIT: u8 = 1 << 4;

#[derive(Serialize, Deserialize)]
pub struct Controls {
    pub button_keys: u8,
    pub direction_keys: u8,
    pub interrupt: bool,
    pub select_button_keys: bool,
    pub select_direction_keys: bool
}

impl Controls {
    pub fn read_byte(&self) -> u8 {
        let mut value = 0;
        if self.select_button_keys {
            value |= SELECT_BUTTON_BIT | self.button_keys;
        }

        if self.select_direction_keys {
            value |= SELECT_DIRECTION_BIT | self.direction_keys;
        }

        value
    }

    pub fn update_input(&mut self, input: &Input) {
        self.button_keys = input.get_button_keys();
        self.direction_keys = input.get_direction_keys();
        self.update_controls();
    }

    pub fn write_byte(&mut self, data: u8) {
        self.select_button_keys = (data & SELECT_BUTTON_BIT) == 0;
        self.select_direction_keys = (data & SELECT_DIRECTION_BIT) == 0;
        self.update_controls();
    }

    fn update_controls(&mut self) {
        let old_values = self.read_byte() & 0xF;
        let mut new_values = 0xF;

        if self.select_button_keys {
            new_values &= self.button_keys;
        }

        if self.select_direction_keys {
            new_values &= self.direction_keys;
        }

        if old_values == 0xF && new_values != 0xF {
            self.interrupt = true;
        }
    }
}

impl Default for Controls {
    fn default() -> Self {
        Controls {
            button_keys: 0b0000_1111,
            direction_keys: 0b0000_1111,
            interrupt: false,
            select_button_keys: true,
            select_direction_keys: true
        }
    }
}