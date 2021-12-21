extern crate caklimas_rust_gameboy as rust_gameboy;

// use ggez::ContextBuilder;
// use ggez::conf::{WindowMode, WindowSetup};
// use ggez::event;
// use std::fs;

// use rust_gameboy::constants::screen::*;

fn main() {
    // let directory = std::env::current_dir().expect("Can't get current directory");
    // let path = directory.join(r"H:\Repos\rust-gameboy-web\public\roms\Dr. Mario.gb");
    // let bytes = fs::read(path).expect("Error reading file");
    // let cb = get_context_builder();
    // let (ctx, event_loop) = &mut cb.build().expect("Error creating event loop");
    // let mut gameboy = rust_gameboy::gameboy::Gameboy::new(bytes, true);
    // event::run(ctx, event_loop, &mut gameboy).expect("Error running event loop");
}

// fn get_context_builder() -> ggez::ContextBuilder {
//     let window_setup: WindowSetup = Default::default();
//     let window_mode: WindowMode = Default::default();
//     let width = (SCREEN_WIDTH as usize) * PIXEL_SIZE;
//     let height = (SCREEN_HEIGHT as usize) * PIXEL_SIZE;
//     ggez::ContextBuilder::new("Rust Gameboy", "caklimas")
//         .window_setup(window_setup.title("Gameboy"))
//         .window_mode(window_mode.dimensions(width as f32, height as f32))
// }
