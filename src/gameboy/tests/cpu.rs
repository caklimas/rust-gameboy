use std::fs;
use super::super::Gameboy;

/*
Passed:
    01-special
    02-interrupts
    03-op sp,hl
    04-op r,imm
    05-op rp
    06-ld r,r
    07-jr,jp,call,ret,rst
    08-misc instrs
    09-op r,r
    10-bit ops
    11-op a,(hl)   
*/

#[test]
fn test() {
    let directory = std::env::current_dir().expect("Can't get current directory");
    let path = directory.join("src/gameboy/tests/roms/02-interrupts.gb");
    let bytes = fs::read(path).expect("Error reading file");
    let mut gameboy = Gameboy::new(bytes, true);
    gameboy.run(None);
}