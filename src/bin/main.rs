extern crate caklimas_rust_gameboy as rust_gameboy;
use std::fs;

fn main() {
    let directory = std::env::current_dir().expect("Can't get current directory");
    let path = directory.join("src/gameboy/tests/roms/02-interrupts.gb");
    let bytes = fs::read(path).expect("Error reading file");
    let mut gameboy = rust_gameboy::gameboy::Gameboy::new(bytes, true);
    loop {
        gameboy.clock();
    }
}