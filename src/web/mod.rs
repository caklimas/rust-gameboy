use wasm_bindgen::prelude::wasm_bindgen;

use crate::{gameboy::Gameboy, input::Input};

#[wasm_bindgen]
pub struct Emulator {
    gameboy: Gameboy,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> Self {
        let gameboy = Gameboy::new(bytes, true);
        Self { gameboy }
    }

    #[inline]
    pub fn clock(&mut self) -> EmulatorState {
        let audio_full = self.gameboy.clock();
        if audio_full {
            return EmulatorState::AudioFull;
        } else if self.gameboy.frame_complete() {
            return EmulatorState::FrameComplete;
        } else {
            return EmulatorState::None;
        }
    }

    #[inline]
    pub fn update_controls(&mut self, input: Input) {
        self.gameboy.update_controls(input);
    }

    #[inline]
    pub fn get_screen(&self) -> Vec<u8> {
        self.gameboy.get_screen().to_vec()
    }

    #[inline]
    pub fn get_audio_buffer(&self) -> Vec<f32> {
        self.gameboy.get_audio_buffer().to_vec()
    }
}

#[wasm_bindgen]
pub enum EmulatorState {
    None,
    FrameComplete,
    AudioFull,
}
