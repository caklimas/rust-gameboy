use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool
}

impl Input {
    pub fn get_button_keys(&self) -> u8 {
        let mut button_keys = 0;
        if !self.a {
            button_keys |= 1 << 0;
        }

        if !self.b {
            button_keys |= 1 << 1;
        }

        if !self.select {
            button_keys |= 1 << 2;
        }

        if !self.start {
            button_keys |= 1 << 3;
        }

        button_keys
    }

    pub fn get_direction_keys(&self) -> u8 {
        let mut direction_keys = 0;
        if !self.right {
            direction_keys |= 1 << 0;
        }

        if !self.left {
            direction_keys |= 1 << 1;
        }

        if !self.up {
            direction_keys |= 1 << 2;
        }

        if !self.down {
            direction_keys |= 1 << 3;
        }

        direction_keys       
    }
}