use std::{cell::RefCell, rc::Rc};

use gameboy::Gameboy;

use wasm_bindgen::{prelude::*, JsCast};
use web::Emulator;
use web_sys::{AudioContext, Window};

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
pub mod web;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
