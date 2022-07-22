use gameboy::Gameboy;

use input::Input;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate bitfield;

pub mod addresses;
pub mod apu;
pub mod cartridge;
pub mod constants;
pub mod controls;
pub mod cpu;
pub mod gameboy;
pub mod gpu;
pub mod input;
pub mod mbc;
pub mod mmu;
pub mod utils;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Emulator {
    cycles: usize,
    gameboy: Gameboy,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> Self {
        let gameboy = Gameboy::new(bytes, true);
        Self { cycles: 0, gameboy }
    }

    pub fn from_save_data(bytes: Vec<u8>, save_data: Vec<u8>) -> Self {
        let gameboy = Gameboy::from_save_data(bytes, save_data, true);
        Self { cycles: 0, gameboy }
    }

    pub fn clock_until_event(&mut self, max_cycles: usize) -> EmulatorState {
        while self.cycles < max_cycles {
            let (cycles, audio_full) = self.gameboy.clock();
            self.cycles += cycles as usize;
            if audio_full {
                return EmulatorState::AudioFull;
            } else if self.gameboy.frame_complete() {
                return EmulatorState::FrameComplete;
            }
        }

        self.cycles -= max_cycles;
        EmulatorState::MaxCycles
    }

    pub fn update_controls(&mut self, input: Input) {
        self.gameboy.update_controls(input);
    }

    pub fn get_screen(&self) -> Vec<u8> {
        self.gameboy.get_screen().to_vec()
    }

    pub fn get_audio_buffer(&self) -> Vec<f32> {
        self.gameboy.get_audio_buffer().to_vec()
    }

    pub fn save(&self) -> Vec<u8> {
        self.gameboy.save()
    }
}

#[wasm_bindgen]
pub enum EmulatorState {
    None,
    FrameComplete,
    AudioFull,
    MaxCycles,
}
