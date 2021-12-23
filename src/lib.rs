use std::mem;
use wasm_bindgen::prelude::*;

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
        let mut audio_buffer = None;
        let mut gb = Box::from_raw(gameboy);
        'running: loop {
            let result = gb.clock();
            if gb.frame_complete() {
                screen = Option::Some(gb.get_screen().to_owned());
            }
            if result.1 {
                audio_buffer = Option::Some(gb.get_audio_buffer());
            }

            if gb.frame_complete() || result.1 {
                break 'running;
            }
        }

        mem::forget(gb);
        Frame::new(audio_buffer, screen)
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
