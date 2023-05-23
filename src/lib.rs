use gameboy::Gameboy;

use input::Input;
use log::Level;
use rom_config::RomConfig;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate log;

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
pub mod rom_config;
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
    pub fn new(bytes: Vec<u8>, rom_config: RomConfig) -> Self {
        init_console_hooks();

        let gameboy = Gameboy::new(bytes, &rom_config);
        Self { cycles: 0, gameboy }
    }

    pub fn from_save_data(bytes: Vec<u8>, save_data: Vec<u8>, rom_config: RomConfig) -> Self {
        init_console_hooks();

        let gameboy = Gameboy::from_save_data(bytes, save_data, &rom_config);
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

    pub fn get_header_info(&self) -> String {
        self.gameboy.get_header_info()
    }

    pub fn get_tiles(&self) -> Vec<u8> {
        self.gameboy.get_tiles()
    }

    pub fn toggle_color(&mut self, use_green_colors: bool) {
        self.gameboy.toggle_color(use_green_colors)
    }
}

#[wasm_bindgen]
pub enum EmulatorState {
    None,
    FrameComplete,
    AudioFull,
    MaxCycles,
}

fn init_console_hooks() {
    console_log::init_with_level(Level::Debug).expect("Error initializing log");
    console_error_panic_hook::set_once();

    info!("It works!");
}
