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
pub fn clock_frame(gameboy: *mut gameboy::Gameboy) -> Vec<u8> {
    unsafe {
        let screen: Vec<u8>;
        let mut gb = Box::from_raw(gameboy);
        'running: loop {
            gb.clock();
            if gb.frame_complete() {
                screen = gb.get_screen().to_owned();
                break 'running;
            }
        }

        mem::forget(gb);
        screen
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
