use gameboy::Gameboy;
use std::{cmp::max, mem};
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioContext, HtmlCanvasElement};

#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_big_array;

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

const SAMPLE_RATE: f32 = 44_100.0;
const SAMPLE_COUNT: u32 = 4096;
const LATENCY: f64 = 0.032;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Emulator {
    audio_context: AudioContext,
    empty_audio_buffers: Vec<AudioBuffer>,
    gameboy: Gameboy,
    timestamp: f64,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>) -> Result<Emulator, JsValue> {
        let audio_context = AudioContext::new()?;
        let gameboy = Gameboy::new(bytes, true);
        Ok(Self {
            audio_context,
            empty_audio_buffers: Vec::new(),
            gameboy,
            timestamp: Default::default(),
        })
    }

    #[inline]
    pub fn clock(&mut self) -> Result<Vec<u8>, JsValue> {
        let screen;
        'running: loop {
            let result = self.gameboy.clock();
            if result.1 {
                let audio_buffer = if self.empty_audio_buffers.len() == 0 {
                    self.audio_context
                        .create_buffer(2, SAMPLE_COUNT, SAMPLE_RATE * 2.0)?
                } else {
                    self.empty_audio_buffers.pop().unwrap()
                };

                let sample = self.gameboy.get_audio_buffer();
                audio_buffer.copy_to_channel(sample, 0)?;

                let node = self.audio_context.create_buffer_source()?;
                node.connect_with_audio_node(&self.audio_context.destination())?;
                node.set_buffer(Option::Some(&audio_buffer));

                let timestamp = self.audio_context.current_time() + LATENCY;
                let play_timestamp = if timestamp >= self.timestamp {
                    timestamp
                } else {
                    self.timestamp
                };

                self.timestamp = play_timestamp + (SAMPLE_COUNT as f64) / 2.0 / SAMPLE_RATE as f64;
                node.start_with_when(play_timestamp)?;
            } else if self.gameboy.frame_complete() {
                screen = self.gameboy.get_screen().to_owned();
                break 'running;
            }
        }

        Ok(screen)
    }

    #[inline]
    pub fn update_controls(&mut self, input: input::Input) {
        self.gameboy.update_controls(input);
    }
}

impl Drop for Emulator {
    fn drop(&mut self) {
        let _ = self.audio_context.close();
    }
}
