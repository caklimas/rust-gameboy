pub mod control;

#[cfg(test)]
mod tests;

use crate::addresses::timer::*;

pub struct Timer {
    pub interrupt_requested: bool,
    counter: u8,
    divider: u8,
    internal_counter: u32,
    modulo: u8
}

impl Timer {
    pub fn clock(&mut self, cycles: u16) {

    }

    pub fn read(&self, address: u16) -> u8 {
        /*
        pub const DIVIDER_REGISTER: u16 = 0xFF04;
pub const TIMER_COUNTER: u16 = 0xFF05;
pub const TIMER_MODULO: u16 = 0xFF06;
pub const TIMER_CONTROL: u16 = 0xFF07;
        */
        match address {
            DIVIDER_REGISTER => 0,
            TIMER_COUNTER => 0,
            TIMER_MODULO => 0,
            TIMER_CONTROL => 0,
            _ => panic!("Invalid timer address 0x{:4X}", address)
        }
    }
}