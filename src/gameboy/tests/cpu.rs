// use std::fs;
// use crate::input::Input;
// use super::super::Gameboy;

// #[test]
// fn test() {
//     let directory = std::env::current_dir().expect("Can't get current directory");
//     let path = directory.join("src/gameboy/tests/roms/02-interrupts.gb");
//     let bytes = fs::read(path).expect("Error reading file");
//     let mut gameboy = Gameboy::new(bytes, true);
//     let input: Input = Default::default();
//     loop {
//         gameboy.clock(&input);
//     }
// }
