use wasm_bindgen::prelude::*;
use std::mem;

#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_big_array;

pub mod addresses;
pub mod cartridge;
pub mod constants;
pub mod cpu;
pub mod gameboy;
pub mod gpu;
pub mod mbc;
pub mod mmu;

#[no_mangle]
#[wasm_bindgen]
pub fn run(bytes: Vec<u8>) -> *mut gameboy::Gameboy {
    let gameboy = gameboy::Gameboy::new(bytes, true);
    let b = Box::new(gameboy); 
    Box::into_raw(b)
}

#[no_mangle]
#[wasm_bindgen]
pub fn get_gameboy(gameboy: *mut gameboy::Gameboy) -> *mut gameboy::Gameboy {
    unsafe {
        let mut gb = Box::from_raw(gameboy);
        gb.number += 1;
        Box::into_raw(gb)
    }
}