pub mod control;

#[cfg(test)]
mod tests;

use crate::addresses::timer::*;
use crate::constants::timer::*;
use control::TimerControl;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Timer {
    pub interrupt_requested: bool,
    control: TimerControl,
    counter: u8,
    cycles: u16,
    divider: u8,
    internal_counter: u32,
    modulo: u8
}

impl Timer {
    pub fn clock(&mut self, cycles: u16) {
        self.cycles += cycles;
        while self.cycles >= DIVIDER_INCREMENT_RATE
        {
            self.divider = self.divider.wrapping_add(1);
            self.cycles -= DIVIDER_INCREMENT_RATE;
        }

        if self.control.timer_enabled() {
            self.internal_counter += cycles as u32;
            while self.internal_counter >= self.control.get_step() {
                self.counter = self.counter.wrapping_add(1);
                if self.counter == 0 {
                    self.counter = self.modulo;
                    self.interrupt_requested = true;
                }

                self.internal_counter -= self.control.get_step();
            }
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            DIVIDER_REGISTER => self.divider,
            TIMER_COUNTER => self.counter,
            TIMER_MODULO => self.modulo,
            TIMER_CONTROL => self.control.get(),
            _ => panic!("Invalid timer address 0x{:4X}", address)
        }
    }

    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            DIVIDER_REGISTER => self.divider = 0,
            TIMER_COUNTER => self.counter = data,
            TIMER_MODULO => self.modulo = data,
            TIMER_CONTROL => self.control.set(data),
            _ => panic!("Invalid timer address 0x{:4X}", address)
        }
    }
}