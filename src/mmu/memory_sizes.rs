use crate::addresses::high_ram::{HIGH_RAM_LOWER, HIGH_RAM_UPPER};

pub const KILOBYTES_1: u16 = 0x0400;
pub const KILOBYTES_2: u16 = 0x0800;
pub const KILOBYTES_4: u16 = 0x1000;
pub const KILOBYTES_8: u16 = 0x2000;
pub const KILOBYTES_16: u16 = 0x4000;
pub const KILOBYTES_32: u16 = 0x8000;
pub const KILOBYTES_64: usize = 0x10000;
pub const KILOBYTES_128: usize = 0x20000;

pub const BOOT_ROM_SIZE: usize = 256;
pub const HIGH_RAM: u16 = (HIGH_RAM_UPPER - HIGH_RAM_LOWER) + 1;
pub const VIDEO_OAM: usize = 0xA0;
