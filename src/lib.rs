use gameboy::Gameboy;
use std::mem;
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

const sample_rate: f32 = 44_100.0;
const sample_count: u32 = 4096;
const latency: f64 = 0.032;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
#[wasm_bindgen]
pub fn run(bytes: Vec<u8>) -> *mut gameboy::Gameboy {
    let gameboy = gameboy::Gameboy::new(bytes, true);
    let b = Box::new(gameboy);
    Box::into_raw(b)
}

#[no_mangle]
#[wasm_bindgen]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn clock_frame(gameboy: *mut gameboy::Gameboy) -> Frame {
    unsafe {
        let mut screen = None;
        let mut gb = Box::from_raw(gameboy);
        'running: loop {
            let result = gb.clock();
            if result.1 {
                let audio_buffer = gb.get_audio_buffer();
            } else if gb.frame_complete() {
                screen = Option::Some(gb.get_screen().to_owned());
            }

            if gb.frame_complete() || result.1 {
                break 'running;
            }
        }

        mem::forget(gb);
        Frame::new(Option::None, screen)
    }
}

#[no_mangle]
#[wasm_bindgen]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn update_controls(gameboy: *mut gameboy::Gameboy, input: input::Input) {
    unsafe {
        let mut gb = Box::from_raw(gameboy);
        gb.update_controls(input);
        mem::forget(gb);
    }
}

#[wasm_bindgen]
pub struct Frame {
    audio_buffer: Option<Vec<f32>>,
    screen: Option<Vec<u8>>,
}

#[wasm_bindgen]
impl Frame {
    pub fn new(audio_buffer: Option<Vec<f32>>, screen: Option<Vec<u8>>) -> Self {
        Self {
            audio_buffer,
            screen,
        }
    }

    pub fn get_audio_buffer_full(&self) -> Option<Vec<f32>> {
        self.audio_buffer.to_owned()
    }

    pub fn get_screen(&self) -> Option<Vec<u8>> {
        self.screen.to_owned()
    }
}

#[wasm_bindgen]
pub struct Emulator {
    audio_context: AudioContext,
    canvas: HtmlCanvasElement,
    empty_audio_buffers: Vec<AudioBuffer>,
    gameboy: Gameboy,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: Vec<u8>, canvas: HtmlCanvasElement) -> Result<Emulator, JsValue> {
        let audio_context = AudioContext::new()?;
        let gameboy = Gameboy::new(bytes, true);
        Ok(Self {
            audio_context,
            canvas,
            empty_audio_buffers: Vec::new(),
            gameboy,
        })
    }

    pub fn clock(&mut self) -> Result<(), JsValue> {
        'running: loop {
            let result = self.gameboy.clock();
            if result.1 {
                let audio_buffer = if self.empty_audio_buffers.len() == 0 {
                    self.audio_context
                        .create_buffer(2, sample_count, sample_rate * 2.0)?
                } else {
                    self.empty_audio_buffers.pop().unwrap()
                };

                let sample = self.gameboy.get_audio_buffer();
                audio_buffer.copy_to_channel(sample, 0);
                let x = self.audio_context.create_buffer(2, 5, 5.0)?;
            } else if self.gameboy.frame_complete() {
                let screen = Option::Some(self.gameboy.get_screen().to_owned());
            }

            if self.gameboy.frame_complete() || result.1 {
                break 'running;
            }
        }

        Ok(())
    }
}

impl Drop for Emulator {
    fn drop(&mut self) {
        let _ = self.audio_context.close();
    }
}
