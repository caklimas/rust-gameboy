extern crate caklimas_rust_gameboy as rust_gameboy;

use ggez::event;
use std::fs;

fn main() {
    let directory = std::env::current_dir().expect("Can't get current directory");
    let path = directory.join("src/gameboy/tests/roms/02-interrupts.gb");
    let bytes = fs::read(path).expect("Error reading file");
    let cb = ggez::ContextBuilder::new("Rust Gameboy", "caklimas");
    let (ctx, event_loop) = &mut cb.build().expect("Error creating event loop");
    let mut gameboy = rust_gameboy::gameboy::Gameboy::new(bytes, true);
    event::run(ctx, event_loop, &mut gameboy).expect("Error running event loop");
}