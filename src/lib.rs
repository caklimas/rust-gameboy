use wasm_bindgen::prelude::*;

#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_big_array;

pub mod addresses;
pub mod cartridge;
pub mod constants;
pub mod cpu;
pub mod gameboy;
pub mod mbc;
pub mod mmu;

#[wasm_bindgen]
pub fn run(bytes: Vec<u8>) {

    if bytes.len() > 0 {
        let mut gameboy = gameboy::Gameboy::new(bytes);
        alert("Loaded file");
    } else {
        alert("This is a test");
    }
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}