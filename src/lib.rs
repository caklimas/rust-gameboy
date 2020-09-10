use wasm_bindgen::prelude::*;

#[macro_use]
extern crate bitfield;

#[macro_use]
extern crate serde_big_array;

pub mod addresses;
pub mod cpu;
pub mod gameboy;
pub mod mmu;

#[wasm_bindgen]
pub fn run() -> u8 {
    let mut cpu: cpu::Cpu = Default::default();
    cpu.clock();
    42
}